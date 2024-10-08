use crate::embeddings::{self, EmbeddingModel};
use crate::rerank::{self, RerankModel};

pub struct Model {
    pub embeddings: embeddings::EmbeddingsRequestBuilder,
    pub rerank: rerank::RerankRequestBuilder,
}

impl Model {
    pub fn new() -> Self {
        Self {
            embeddings: embeddings::EmbeddingsRequestBuilder::new(),
            rerank: rerank::RerankRequestBuilder::new(),
        }
    }

    pub fn embeddings(&self) -> &embeddings::EmbeddingsRequestBuilder {
        &self.embeddings
    }

    pub fn rerank(&self) -> &rerank::RerankRequestBuilder {
        &self.rerank
    }

    pub fn builder() -> ModelBuilder {
        ModelBuilder::new()
    }
}

pub struct ModelBuilder {
    embeddings: Option<embeddings::EmbeddingsRequestBuilder>,
    rerank: Option<rerank::RerankRequestBuilder>,
}

impl ModelBuilder {
    pub fn new() -> Self {
        Self {
            embeddings: None,
            rerank: None,
        }
    }

    pub fn embeddings(mut self, embeddings: embeddings::EmbeddingsRequestBuilder) -> Self {
        self.embeddings = Some(embeddings);
        self
    }

    pub fn rerank(mut self, rerank: rerank::RerankRequestBuilder) -> Self {
        self.rerank = Some(rerank);
        self
    }

    pub fn build(self) -> Model {
        Model {
            embeddings: self
                .embeddings
                .unwrap_or_else(embeddings::EmbeddingsRequestBuilder::new),
            rerank: self
                .rerank
                .unwrap_or_else(rerank::RerankRequestBuilder::new),
        }
    }
}

pub use EmbeddingModel as Embedding;
pub use RerankModel as ReRank;
