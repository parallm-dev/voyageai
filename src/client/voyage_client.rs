use crate::builder::VoyageBuilder;
use crate::client::RateLimiter;
use crate::config::VoyageConfig;
use log::{debug, info};
use std::sync::Arc;

/// The main client for interacting with the Voyage AI API.
///
/// This client provides access to both embedding and reranking functionalities,
/// and handles rate limiting automatically.
#[derive(Debug, Clone)]
pub struct VoyageAiClient {
    /// Configuration for the client.
    pub config: VoyageConfig,
    /// HTTP client for making API requests.
    pub client: reqwest::Client,
    /// Client for embedding operations.
    pub embeddings_client: crate::client::embeddings_client::EmbeddingClient,
    /// Client for reranking operations.
    pub rerank_client: crate::client::rerank_client::RerankClient,
    /// Rate limiter to prevent exceeding API usage limits.
    pub rate_limiter: Arc<RateLimiter>,
}

impl VoyageAiClient {
    /// Creates a new `VoyageAiClient` instance with the given configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - The configuration for the client.
    ///
    /// # Returns
    ///
    /// A new `VoyageAiClient` instance.
    pub fn new(config: VoyageConfig) -> Self {
        info!("Creating new VoyageAiClient");
        let rate_limiter = Arc::new(RateLimiter::new());
        let client = reqwest::Client::new();

        debug!("Initializing EmbeddingClient and RerankClient");
        let embeddings_client = crate::client::embeddings_client::EmbeddingClient::new(
            config.clone(),
            rate_limiter.clone(),
        );
        let rerank_client =
            crate::client::rerank_client::RerankClient::new(config.clone(), rate_limiter.clone());

        debug!("VoyageAiClient initialization complete");
        Self {
            embeddings_client,
            rerank_client,
            config,
            client,
            rate_limiter,
        }
    }

    /// Returns a new `VoyageBuilder` instance for constructing a `VoyageAiClient`.
    ///
    /// # Returns
    ///
    /// A new `VoyageBuilder` instance.
    pub fn builder() -> VoyageBuilder {
        debug!("Creating new VoyageBuilder");
        VoyageBuilder::new()
    }

    /// Returns a reference to the embedding client.
    ///
    /// # Returns
    ///
    /// A reference to the `EmbeddingClient`.
    pub fn embeddings(&self) -> &crate::client::embeddings_client::EmbeddingClient {
        debug!("Accessing EmbeddingClient");
        &self.embeddings_client
    }

    /// Returns a reference to the reranking client.
    ///
    /// # Returns
    ///
    /// A reference to the `RerankClient`.
    pub fn rerank(&self) -> &crate::client::rerank_client::RerankClient {
        debug!("Accessing RerankClient");
        &self.rerank_client
    }
}
