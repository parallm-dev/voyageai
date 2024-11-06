use crate::{
    builder::embeddings::EmbeddingsRequestBuilder,
    client::{SearchRequest, VoyageAiClient},
    RerankRequestBuilder,
    SearchRequestBuilder,
    config::VoyageConfig,
    errors::VoyageError,
    models::{
        rerank::RerankRequest,
        search::{SearchQuery, SearchType},
    },
};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct VoyageBuilder {
    config: Option<VoyageConfig>,
    embeddings: Option<EmbeddingsRequestBuilder>,
    rerank: Option<RerankRequestBuilder>,
    search: Option<SearchRequestBuilder>,
}


impl VoyageBuilder {
    pub fn new() -> VoyageBuilder {
        VoyageBuilder {
            config: None,
            embeddings: None,
            rerank: None,
            search: None
        }
    }

    pub fn with_api_key(mut self, api_key: impl Into<String>) -> VoyageBuilder {
        self.config = Some(VoyageConfig::new(api_key.into()));
        self
    }

    pub fn build(&self) -> Result<VoyageAiClient, VoyageError> {
        let config = self.config.as_ref()
            .ok_or_else(|| VoyageError::BuilderError("API key is required".to_string()))?;
        Ok(VoyageAiClient::with_key(config.api_key()))
    }
}

