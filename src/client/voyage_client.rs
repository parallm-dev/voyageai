use crate::builder::VoyageBuilder;
use crate::embeddings_client::EmbeddingsClient;

#[derive(Debug, Clone)]
pub struct VoyageAiClient {
    pub api_key: String,
    pub client: reqwest::Client,
    pub embeddings_client: EmbeddingsClient,
    pub rate_limiter: RateLimiter,
    pub rerank_client: RerankClient,
}

impl VoyageAiClient {
    pub fn builder() -> VoyageBuilder {
        VoyageBuilder::new()
    }

    pub fn embeddings(&self) -> &EmbeddingsClient {
        &self.embeddings_client
    }

    pub fn rerank(&self) -> &crate::rerank_client::RerankClient {
        &self.rerank_client
    }

    // ... additional methods needed ...
}
