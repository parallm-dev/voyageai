use crate::models::{embeddings::EmbeddingModel, search::SearchModel, RerankModel};
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

#[derive(Debug, Clone, Default)]
pub struct VoyageConfig {
    pub api_key: String,
    pub base_url: String,
    pub search_model: SearchModel,
    pub embedding_model: EmbeddingModel,
}

impl VoyageConfig {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            base_url: "https://api.voyageai.com/v1".to_string(),
            search_model: SearchModel::default(),
            embedding_model: EmbeddingModel::default(),
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

fn default_embedding_model() -> EmbeddingModel {
    EmbeddingModel::Voyage3
}

#[allow(dead_code)]
fn default_reranker_model() -> RerankModel {
    RerankModel::Rerank2
}
