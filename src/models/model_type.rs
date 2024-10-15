use crate::models::{EmbeddingModel, RerankModel};

pub enum ModelType {
    Rerank(RerankModel),
    Embedding(EmbeddingModel),
}
