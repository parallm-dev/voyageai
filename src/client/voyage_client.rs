use crate::builder::{VoyageBuilder, EmbeddingsRequestBuilder, RerankRequestBuilder};
use crate::config::ClientConfig;
use crate::limiter::RateLimiter;

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
        EmbeddingsRequestBuilder::new().voyage(self.clone())
    }

    pub fn rerank(&self) -> RerankRequestBuilder {
        RerankRequestBuilder::new().voyage(self.clone())
    }

    // ... Add any additional methods needed ...
}

impl Clone for VoyageAiClient {
    fn clone(&self) -> Self {
        Self {
            api_key: self.api_key.clone(),
            client: self.client.clone(),
            rate_limiter: self.rate_limiter.clone(),
        }
    }
}
