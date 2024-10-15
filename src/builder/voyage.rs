use crate::client::client_limiter::RateLimiter;
use crate::client::embeddings_client::EmbeddingClient;
use crate::client::rerank_client::RerankClient;
use crate::config::VoyageConfig;
use crate::errors::VoyageBuilderError;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct VoyageAiClient {
    pub(crate) embeddings_client: EmbeddingClient,
    pub(crate) rerank_client: RerankClient,
    #[allow(dead_code)]
    pub(crate) rate_limiter: Arc<RateLimiter>,
}

impl VoyageAiClient {
    pub fn builder() -> VoyageBuilder {
        VoyageBuilder::new()
    }

    pub fn embeddings(&self) -> &EmbeddingClient {
        &self.embeddings_client
    }

    pub fn rerank(&self) -> &RerankClient {
        &self.rerank_client
    }

    pub fn embeddings_mut(&mut self) -> &mut EmbeddingClient {
        &mut self.embeddings_client
    }

    pub fn rerank_mut(&mut self) -> &mut RerankClient {
        &mut self.rerank_client
    }
}

impl Default for VoyageAiClient {
    fn default() -> Self {
        let config = VoyageConfig::default();
        let rate_limiter = Arc::new(RateLimiter::new());
        Self {
            embeddings_client: EmbeddingClient::new(config.clone(), rate_limiter.clone()),
            rerank_client: RerankClient::new(config, rate_limiter.clone()),
            rate_limiter,
        }
    }
}

#[derive(Debug, Default)]
pub struct VoyageBuilder {
    api_key: Option<String>,
    client: Option<reqwest::Client>,
    config: Option<VoyageConfig>,
}

impl VoyageBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn api_key<T: Into<String>>(mut self, api_key: T) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    pub fn client(mut self, client: reqwest::Client) -> Self {
        self.client = Some(client);
        self
    }

    pub fn config(mut self, config: VoyageConfig) -> Self {
        self.config = Some(config);
        self
    }

    pub fn build(self) -> Result<VoyageAiClient, VoyageBuilderError> {
        let api_key = match self.api_key {
            Some(key) if !key.is_empty() => key,
            _ => std::env::var("VOYAGE_API_KEY").map_err(|_| VoyageBuilderError::ApiKeyNotSet)?,
        };

        let _client = self.client.unwrap_or_default();
        let config = self.config.unwrap_or_else(|| VoyageConfig::new(api_key));
        let rate_limiter = Arc::new(RateLimiter::new());

        let embeddings_client = EmbeddingClient::new(config.clone(), rate_limiter.clone());
        let rerank_client = RerankClient::new(config, rate_limiter.clone());

        Ok(VoyageAiClient {
            embeddings_client,
            rerank_client,
            rate_limiter,
        })
    }
}

pub use crate::builder::embeddings::EmbeddingsRequest;
pub use crate::builder::rerank::RerankRequest;
