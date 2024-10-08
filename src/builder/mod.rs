mod embeddings;
mod rerank;
mod voyage;

pub use embeddings::EmbeddingsRequestBuilder;
pub use rerank::RerankRequestBuilder;
pub use voyage::VoyageBuilder;

use crate::voyage_config::ClientConfig;
use crate::voyage_errors::VoyageBuilderError;
use crate::voyage_limiter::RateLimiter;
use crate::VoyageAiClient;

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
}
