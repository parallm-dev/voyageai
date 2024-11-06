use crate::{
    builder::embeddings::EmbeddingsRequestBuilder,
    client::{SearchRequest, VoyageAiClient},
    RerankRequestBuilder,
    SearchRequestBuilder,
    config::VoyageConfig,
    errors::VoyageError,
    models::{
        rerank::RerankRequest,
        search::{SearchQuery, SearchType},
    },
};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct VoyageBuilder {
    config: Option<VoyageConfig>,
    embeddings: Option<EmbeddingsRequestBuilder>,
    rerank: Option<RerankRequestBuilder>,
    search: Option<SearchRequestBuilder>,
}


impl VoyageBuilder {
    pub fn new() -> VoyageBuilder {
        VoyageBuilder {
            config: None,
            embeddings: None,
            rerank: None,
            search: None
        }
    }

    pub fn with_api_key(mut self, api_key: impl Into<String>) -> VoyageBuilder {
        self.config = Some(VoyageConfig::new(api_key.into()));
        self
    }

    pub fn build(&self) -> Result<Arc<RwLock<VoyageAiClient>>, &'static str> {
        let config = self.config.as_ref().ok_or("API key is required")?;
        Ok(Arc::new(RwLock::new(VoyageAiClient::with_key(
            config.api_key(),
        ))))
    }

    pub async fn search(&self, query: impl Into<String>) -> Result<Vec<f32>, VoyageError> {
        let client = self
            .build()
            .map_err(|e| VoyageError::BuilderError(e.to_string()))?;
        let client = client.write().await;

        let search_query = SearchQuery {
            query: query.into(),
            model: Default::default(),
            max_results: None,
            num_results: Some(10),
            include_metadata: None,
        };

        let request = SearchRequest {
            query: search_query,
            documents: None,
            embeddings: None,
            model: Default::default(),
            top_k: None,
            search_type: SearchType::Similarity,
        };

        let results = client
            .search(request)
            .await
            .map_err(|e| VoyageError::BuilderError(e.to_string()))?;
        Ok(results
            .into_iter()
            .map(|r| r.score as f32)
            .collect::<Vec<f32>>())
    }

    pub async fn rerank(
        &self,
        query: impl Into<String>,
        documents: Vec<String>,
    ) -> Result<Vec<f32>, VoyageError> {
        let client = self
            .build()
            .map_err(|e| VoyageError::BuilderError(e.to_string()))?;
        let client = client.write().await;

        let rerank_request = RerankRequest::new(query.into(), documents, Default::default(), None)
            .map_err(VoyageError::from)?;

        client
            .rerank(rerank_request)
            .await
            .map_err(|e| VoyageError::BuilderError(e.to_string()))
            .map(|response| {
                response
                    .data
                    .into_iter()
                    .map(|r| r.relevance_score as f32)
                    .collect::<Vec<f32>>()
            })
    }

    pub async fn embed(&self, text: impl Into<String>) -> Result<Vec<f32>, VoyageError> {
        let client = self
            .build()
            .map_err(|e| VoyageError::BuilderError(e.to_string()))?;
        let client = client.write().await;

        let request = EmbeddingsRequestBuilder::new()
            .document(text)
            .model(Default::default())
            .build()
            .map_err(|e| VoyageError::BuilderError(e.to_string()))?;

        let input = request.input;

        let response = client
            .create_embeddings(input)
            .await
            .map_err(|e| VoyageError::BuilderError(e.to_string()))?;

        Ok(response
            .data
            .into_iter()
            .next()
            .map(|d| d.embedding)
            .unwrap_or_default())
    }

    pub async fn embed_batch(&self, texts: Vec<String>) -> Result<Vec<Vec<f32>>, VoyageError> {
        let client = self
            .build()
            .map_err(|e| VoyageError::BuilderError(e.to_string()))?;
        let client = client.write().await;

        let request = EmbeddingsRequestBuilder::new()
            .documents(texts)
            .model(Default::default())
            .build()
            .map_err(|e| VoyageError::BuilderError(e.to_string()))?;

        let input = request.input;

        let response = client
            .create_embeddings(input)
            .await
            .map_err(|e| VoyageError::BuilderError(e.to_string()))?;

        Ok(response.data.into_iter().map(|d| d.embedding).collect())
    }
}

