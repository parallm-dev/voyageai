use crate::{
    builder::{
        embeddings::EmbeddingsRequestBuilder,
        rerank::RerankRequestBuilder,
        search::SearchRequestBuilder,
    },
    client::{
        embeddings_client::EmbeddingClient,
        rerank_client::DefaultRerankClient,
        search_client::SearchClient,
        RateLimiter,
        voyage_client::{VoyageAiClient, VoyageAiClientConfig},
    },
    config::VoyageConfig,
    errors::VoyageError,
};
use std::sync::Arc;

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

    pub fn build(self) -> Result<VoyageAiClient, VoyageError> {
        let config = self.config
            .ok_or_else(|| VoyageError::BuilderError("API key is required".to_string()))?;
        
        Ok(VoyageAiClient {
            config: VoyageAiClientConfig {
                config,
                embeddings_client: Arc::new(EmbeddingClient::new(config.clone(), Arc::new(RateLimiter::new()))),
                rerank_client: Arc::new(DefaultRerankClient::new(config.clone(), Arc::new(RateLimiter::new()))),
                search_client: Arc::new(SearchClient::new(
                    EmbeddingClient::new(config.clone(), Arc::new(RateLimiter::new())),
                    DefaultRerankClient::new(config.clone(), Arc::new(RateLimiter::new()))
                )),
            }
        })
    }
}

