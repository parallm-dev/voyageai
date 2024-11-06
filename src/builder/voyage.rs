use crate::{
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
}

impl VoyageBuilder {
    pub fn new() -> VoyageBuilder {
        VoyageBuilder {
            config: None,
        }
    }

    pub fn with_api_key(mut self, api_key: impl Into<String>) -> VoyageBuilder {
        self.config = Some(VoyageConfig::new(api_key.into()));
        self
    }

    pub fn build(self) -> Result<VoyageAiClient, VoyageError> {
        let config = self.config
            .ok_or_else(|| VoyageError::BuilderError("API key is required".to_string()))?;
        
        let rate_limiter = Arc::new(RateLimiter::new());
        Ok(VoyageAiClient {
            config: VoyageAiClientConfig {
                config: config.clone(),
                embeddings_client: Arc::new(EmbeddingClient::new(config.clone(), rate_limiter.clone())),
                rerank_client: Arc::new(DefaultRerankClient::new(config.clone(), rate_limiter.clone())),
                search_client: Arc::new(SearchClient::new(
                    EmbeddingClient::new(config.clone(), rate_limiter.clone()),
                    DefaultRerankClient::new(config, rate_limiter)
                )),
            }
        })
    }
}

