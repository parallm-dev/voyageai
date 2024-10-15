use crate::builder::voyage::VoyageBuilder;
use crate::client::RateLimiter;
use crate::config::VoyageConfig;
use log::{debug, info};
use std::sync::Arc;

use crate::client::embeddings_client::EmbeddingClient;
use crate::client::rerank_client::RerankClient;
use crate::client::search_client::SearchClient;

pub struct VoyageAiClient {
    pub config: VoyageConfig,
    pub client: reqwest::Client,
    pub embeddings_client: EmbeddingClient,
    pub rerank_client: RerankClient,
    pub search_client: SearchClient,
    pub rate_limiter: Arc<RateLimiter>,
}

impl VoyageAiClient {
    pub fn new(config: VoyageConfig) -> Self {
        info!("Creating new VoyageAiClient");
        let rate_limiter = Arc::new(RateLimiter::new());
        let client = reqwest::Client::new();

        debug!("Initializing EmbeddingClient and RerankClient");
        let embeddings_client = EmbeddingClient::new(config.clone(), rate_limiter.clone());
        let rerank_client = RerankClient::new(config.clone(), rate_limiter.clone());
        let search_client = SearchClient::new(embeddings_client.clone(), rerank_client.clone());

        debug!("VoyageAiClient initialization complete");
        Self {
            embeddings_client,
            rerank_client,
            search_client,
            config,
            client,
            rate_limiter,
        }
    }

    pub fn builder() -> VoyageBuilder {
        debug!("Creating new VoyageBuilder");
        VoyageBuilder::new()
    }

    pub fn embeddings(&self) -> &EmbeddingClient {
        debug!("Accessing EmbeddingClient");
        &self.embeddings_client
    }

    pub fn rerank(&self) -> &RerankClient {
        debug!("Accessing RerankClient");
        &self.rerank_client
    }

    pub fn search(&self) -> &SearchClient {
        debug!("Accessing SearchClient");
        &self.search_client
    }
}
