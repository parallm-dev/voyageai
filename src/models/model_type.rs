use crate::models::{EmbeddingModel, RerankModel};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum ModelType {
    Rerank(RerankModel),
    Embedding(EmbeddingModel),
}

impl ModelType {
    pub fn as_str(&self) -> &str {
        match self {
            ModelType::Rerank(_) => "rerank",
            ModelType::Embedding(_) => "embedding",
        }
    }
}
