use crate::client::client_limiter::RateLimiter;
use crate::config::VoyageConfig;
use crate::errors::VoyageBuilderError;

#[derive(Clone, Debug)]
pub struct VoyageAiClient {
    pub(crate) embeddings_client: crate::client::embeddings_client::EmbeddingClient,
    pub(crate) rerank_client: crate::client::rerank_client::RerankClient,
}

impl VoyageAiClient {
    pub fn builder() -> VoyageBuilder {
        VoyageBuilder::new()
    }

    pub fn embeddings(&self) -> &crate::client::embeddings_client::EmbeddingClient {
        &self.embeddings_client
    }

    pub fn rerank(&self) -> &crate::client::rerank_client::RerankClient {
        &self.rerank_client
    }

    pub fn embeddings_mut(&mut self) -> &mut crate::client::embeddings_client::EmbeddingClient {
        &mut self.embeddings_client
    }

    pub fn rerank_mut(&mut self) -> &mut crate::client::rerank_client::RerankClient {
        &mut self.rerank_client
    }
}

impl Default for VoyageAiClient {
    fn default() -> Self {
        let config = VoyageConfig::default();
        Self {
            embeddings_client: crate::client::embeddings_client::EmbeddingClient::new(
                config.clone(),
            ),
            rerank_client: crate::client::rerank_client::RerankClient::new(String::new()),
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
        let api_key = self
            .api_key
            .or_else(|| std::env::var("VOYAGE_API_KEY").ok())
            .or_else(|| std::env::var("VOYAGEAI_API_KEY").ok())
            .ok_or(VoyageBuilderError::ApiKeyNotSet)?;

        let _client = self.client.unwrap_or_default();
        let config = self.config.unwrap_or_default();

        let _rate_limiter = RateLimiter::new(config.rate_limit_duration);

        let embeddings_client =
            crate::client::embeddings_client::EmbeddingClient::new(config.clone());
        let rerank_client = crate::client::rerank_client::RerankClient::new(api_key.clone());

        Ok(VoyageAiClient {
            embeddings_client,
            rerank_client,
        })
    }
}

// Removed unused functions

pub use crate::builder::embeddings::EmbeddingsRequest;
pub use crate::builder::rerank::RerankRequest;
