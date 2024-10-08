use crate::builder::VoyageBuilder;
use crate::client::RateLimiter;

#[derive(Debug, Clone)]
pub struct VoyageAiClient {
    pub api_key: String,
    pub client: reqwest::Client,
    pub embeddings_client: crate::client::embeddings_client::EmbeddingClient,
    pub rerank_client: crate::client::rerank_client::RerankClient,
    pub rate_limiter: RateLimiter,
}

impl VoyageAiClient {
    pub fn builder() -> VoyageBuilder {
        VoyageBuilder::new()
    }

    pub fn embeddings(&self) -> &crate::client::embeddings_client::EmbeddingsClient {
        &self.embeddings_client
    }

    pub fn rerank(&self) -> &crate::client::rerank_client::RerankClient {
        &self.rerank_client
    }

    // ... additional methods needed ...
}
