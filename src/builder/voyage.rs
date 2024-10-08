use crate::client::client_limiter::RateLimiter;
use crate::client::{EmbeddingsRequest, EmbeddingsResponse};
use crate::client::{RerankRequest, RerankResponse};
use crate::config::VoyageConfig;
use crate::errors::VoyageBuilderError;
use crate::models::EmbeddingModel;
use crate::models::RerankModel;
use crate::EmbeddingsRequestBuilder;
use crate::RerankRequestBuilder;

#[derive(Clone, Debug)]
pub struct VoyageAiClient {
    pub(crate) api_key: String,
    pub(crate) client: reqwest::Client,
    pub(crate) rate_limiter: RateLimiter,
}

impl VoyageAiClient {
    pub fn builder() -> VoyageBuilder {
        VoyageBuilder::new()
    }

    pub fn embeddings(&self) -> EmbeddingsRequestBuilder {
        EmbeddingsRequestBuilder::new()
    }

    pub fn rerank(&self) -> RerankRequestBuilder {
        RerankRequestBuilder::new(self.clone())
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

        let client = self.client.unwrap_or_default();
        let config = self.config.unwrap_or_else(VoyageConfig::default);

        let rate_limiter = RateLimiter::new(config.rate_limit_duration);

        Ok(VoyageAiClient {
            api_key,
            client,
            rate_limiter,
        })
    }
}

pub fn embeddings() -> EmbeddingsRequestBuilder {
    EmbeddingsRequestBuilder::new()
}

pub fn rerank() -> RerankRequestBuilder {
    RerankRequestBuilder::new()
}
