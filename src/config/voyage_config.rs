use crate::models::{EmbeddingModel, RerankModel, search::SearchModel};
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub enum Model {
    Embedding(EmbeddingModel),
    Rerank(RerankModel),
    Search(SearchModel),
}

impl Default for Model {
    fn default() -> Self {
        Model::Embedding(default_embedding_model())
    }
}

#[derive(Debug, Clone)]
pub struct VoyageConfig {
    pub api_key: String,
    pub base_url: String,
    pub search_model: SearchModel,
}

impl VoyageConfig {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            base_url: "https://api.voyageai.com/v1".to_string(),
            search_model: SearchModel::default(),
        }
    }

    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    pub fn api_key(&self) -> &str {
        &self.api_key
    }
}

impl Default for VoyageConfig {
    fn default() -> Self {
        let api_key = std::env::var("VOYAGE_API_KEY").unwrap_or_default();
        Self {
            api_key,
            base_url: "https://api.voyageai.com/v1".to_string(),
            search_model: SearchModel::default(),
        }
    }
}

fn default_embedding_model() -> EmbeddingModel {
    EmbeddingModel::Voyage3
}

#[allow(dead_code)]
fn default_reranker_model() -> RerankModel {
    RerankModel::Rerank2
}
