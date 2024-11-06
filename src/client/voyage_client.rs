use std::sync::Arc;
use tokio::sync::RwLock;
use async_trait::async_trait;
use crate::{
    builder::{
        search::SearchRequest,
        embeddings::EmbeddingsRequestBuilder,
    },
    client::{
        embeddings_client::EmbeddingClient, 
        rerank_client::DefaultRerankClient,
        search_client::SearchClient, 
        RateLimiter, 
        RerankClient,
    },
    config::VoyageConfig,
    errors::VoyageError,
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

#[async_trait]
impl VoyageAiClientExt for Arc<RwLock<VoyageAiClient>> {
    async fn embed(
        &self,
        input: impl Into<EmbeddingsInput> + Send,
    ) -> Result<EmbeddingsResponse, Box<dyn std::error::Error>> {
        self.read().await.embed(input).await
    }

    async fn rerank(
        &self,
        request: RerankRequest,
    ) -> Result<RerankResponse, Box<dyn std::error::Error>> {
        self.read().await.rerank(request).await.map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }

    async fn search(
        &self,
        request: SearchRequest,
    ) -> Result<Vec<SearchResult>, Box<dyn std::error::Error>> {
        self.read().await.search(request).await.map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
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

    pub async fn embed(
        &self,
        input: impl Into<EmbeddingsInput>,
    ) -> Result<EmbeddingsResponse, Box<dyn std::error::Error>> {
        debug!("Accessing EmbeddingClient for embeddings request");

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

    pub async fn embed_text(&self, text: impl Into<String>) -> Result<Vec<f32>, VoyageError> {
        let request = EmbeddingsRequestBuilder::new()
            .document(text)
            .model(self.config.config.embedding_model)
            .build()?;

        let response = self.embeddings(request).await?;
        Ok(response.data.into_iter().next()
            .map(|d| d.embedding)
            .unwrap_or_default())
    }

    pub async fn embed_batch(&self, texts: Vec<String>) -> Result<Vec<Vec<f32>>, VoyageError> {
        let request = EmbeddingsRequestBuilder::new()
            .documents(texts)
            .model(self.config.config.embedding_model)
            .build()?;

        let response = self.embeddings(request.input).await?;
        Ok(response.data.into_iter().map(|d| d.embedding).collect())
    }

    pub async fn rerank(&self, request: RerankRequest) -> Result<RerankResponse, VoyageError> {
        debug!("Accessing RerankClient");
        self.config.rerank_client.rerank(&request).await
    }

    pub async fn search(&self, request: SearchRequest) -> Result<Vec<SearchResult>, VoyageError> {
        debug!("Accessing SearchClient");
        self.config.search_client.search(&request).await
            .map_err(VoyageError::from)
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
        if let Ok(response) = self.client.embeddings(input).await {
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
