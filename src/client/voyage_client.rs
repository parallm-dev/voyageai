use std::sync::Arc;
use tokio::sync::RwLock;
use async_trait::async_trait;

use crate::{
    builder::search::SearchRequest,
    models::model_type::ModelType,
    client::{
        embeddings_client::EmbeddingClient, rerank_client::DefaultRerankClient,
        search_client::SearchClient, RateLimiter, RerankClient,
    },
    config::VoyageConfig,
    models::{
        embeddings::{EmbeddingsInput, EmbeddingsRequest, EmbeddingsResponse},
        rerank::{RerankModel, RerankRequest, RerankResponse},
        search::{SearchModel, SearchQuery, SearchType},
    },
};

use super::SearchResult;
use log::{debug, info};

#[derive(Clone)]
pub struct VoyageAiClientConfig {
    config: VoyageConfig,
    embeddings_client: Arc<EmbeddingClient>,
    rerank_client: Arc<DefaultRerankClient>,
    search_client: Arc<SearchClient>,
}

use crate::traits::voyage::VoyageAiClientExt;
use crate::models::embeddings::EmbeddingsRequest;

#[async_trait]
impl VoyageAiClientExt for Arc<RwLock<VoyageAiClient>> {
    pub async fn create_embeddings(&self, request: EmbeddingsRequest) -> Result<EmbeddingsResponse, VoyageError> {
        let response = self.client
            .post(&self.config.embeddings_url())
            .json(&request)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            Err(VoyageError::from_response(response).await)
        }
    }

    async fn rerank(
        &self,
        request: RerankRequest,
    ) -> Result<RerankResponse, Box<dyn std::error::Error>> {
        self.read().await.rerank(request).await
    }

    async fn search(
        &self,
        request: SearchRequest,
    ) -> Result<Vec<SearchResult>, Box<dyn std::error::Error>> {
        self.read().await.search(request).await
    }
}

#[derive(Clone)]
pub struct VoyageAiClient {
    config: VoyageAiClientConfig,
}

impl Default for VoyageAiClient {
    fn default() -> Self {
        Self::new()
    }
}

impl VoyageAiClient {
    pub fn new_with_config(config: VoyageConfig) -> Self {
        info!("Creating new VoyageAiClient");
        let rate_limiter = Arc::new(RateLimiter::new());
        let embeddings_client =
            Arc::new(EmbeddingClient::new(config.clone(), rate_limiter.clone()));
        let rerank_client = Arc::new(DefaultRerankClient::new(
            config.clone(),
            rate_limiter.clone(),
        ));
        let search_client = Arc::new(SearchClient::new(
            (*embeddings_client).clone(),
            (*rerank_client).clone(),
        ));

        let client_config = VoyageAiClientConfig {
            config,
            embeddings_client,
            rerank_client,
            search_client,
        };

        Self {
            config: client_config,
        }
    }

    pub fn new() -> Self {
        info!("Creating new VoyageAiClient from environment");
        let api_key = std::env::var("VOYAGE_API_KEY")
            .or_else(|_| std::env::var("VOYAGEAI_API_KEY"))
            .expect("API key must be set");
        Self::new_with_config(VoyageConfig::new(api_key))
    }

    pub fn with_key(api_key: impl Into<String>) -> Self {
        info!("Creating new VoyageAiClient with provided key");
        Self::new_with_config(VoyageConfig::new(api_key.into()))
    }

    pub async fn create_embeddings(
        &self,
        input: impl Into<EmbeddingsInput>,
    ) -> Result<EmbeddingsResponse, Box<dyn std::error::Error>> {
        debug!("Accessing EmbeddingClient for {} model", ModelType::Embedding(self.config.config.embedding_model).as_str());

        let request = EmbeddingsRequest {
            input: input.into(),
            model: self.config.config.embedding_model,
            input_type: None,
            truncation: None,
            encoding_format: None,
        };

        self.config
            .embeddings_client
            .create_embedding(&request)
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }

    pub async fn embed_text(
        &self,
        text: impl Into<String>,
    ) -> Result<Vec<Vec<f32>>, Box<dyn std::error::Error>> {
        let text = text.into();
        let input = EmbeddingsInput::from(vec![text]);
        let response = self.create_embeddings(input).await?;
        Ok(response.data.into_iter().map(|e| e.embedding).collect())
    }

    pub async fn embed_documents(
        &self,
        documents: Vec<impl Into<String>>,
    ) -> Result<Vec<Vec<f32>>, Box<dyn std::error::Error>> {
        let docs: Vec<String> = documents.into_iter().map(|d| d.into()).collect();
        let input = EmbeddingsInput::from(docs);
        let response = self.create_embeddings(input).await?;
        Ok(response.data.into_iter().map(|e| e.embedding).collect())
    }

    pub async fn rerank(
        &self,
        request: RerankRequest,
    ) -> Result<RerankResponse, Box<dyn std::error::Error>> {
        debug!("Accessing RerankClient");
        let response = self.config.rerank_client.rerank(&request).await?;
        Ok(response)
    }

    pub async fn search(
        &self,
        request: SearchRequest,
    ) -> Result<Vec<SearchResult>, Box<dyn std::error::Error>> {
        debug!("Accessing SearchClient");
        let search_client = &self.config.search_client;
        let search_results = search_client.clone().search(&request).await?;
        Ok(search_results)
    }

    pub fn chain(&self) -> ChainedOperationBuilder {
        ChainedOperationBuilder::new(self)
    }

    pub fn build(&self) -> Self {
        self.clone()
    }
}

pub struct ChainedOperationBuilder<'a> {
    client: &'a VoyageAiClient,
    embedded_docs: Option<Vec<Vec<f32>>>,
    reranked_docs: Option<Vec<String>>,
    search_results: Option<SearchRequest>,
}

impl<'a> ChainedOperationBuilder<'a> {
    pub fn new(client: &'a VoyageAiClient) -> Self {
        Self {
            client,
            embedded_docs: None,
            reranked_docs: None,
            search_results: None,
        }
    }

    pub async fn embed_documents(mut self, input: impl Into<EmbeddingsInput>) -> Self {
        if let Ok(response) = self.client.create_embeddings(input).await {
            self.embedded_docs = Some(response.data.into_iter().map(|e| e.embedding).collect());
        }
        self
    }

    pub async fn rerank_documents(mut self, query: &str, documents: Vec<String>) -> Self {
        let rerank_request =
            RerankRequest::new(query.to_string(), documents, RerankModel::Rerank2, None)
                .expect("Invalid rerank request");

        if let Ok(response) = self.client.rerank(rerank_request).await {
            self.reranked_docs = Some(
                response
                    .data
                    .into_iter()
                    .filter_map(|r| r.document)
                    .collect(),
            );
        }
        self
    }

    pub async fn search(mut self, query: impl Into<String>) -> Self {
        let search_query = SearchQuery {
            query: query.into(),
            model: SearchModel::default(),
            max_results: None,
            num_results: Some(10),
            include_metadata: Some(false),
        };

        self.search_results = Some(SearchRequest {
            query: search_query,
            documents: Some(self.reranked_docs.clone().unwrap_or_default()),
            embeddings: self.embedded_docs.clone(),
            model: SearchModel::default(),
            top_k: None,
            search_type: SearchType::Similarity,
        });
        self
    }

    pub async fn execute(self) -> Option<SearchRequest> {
        self.search_results
    }
}
