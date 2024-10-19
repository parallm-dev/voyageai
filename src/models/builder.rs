use crate::EmbeddingsRequestBuilder;
use crate::RerankRequestBuilder;
use crate::models::search::SearchRequestBuilder; // Add this line

pub struct Model {
    pub embeddings: EmbeddingsRequestBuilder,
    pub rerank: RerankRequestBuilder,
    pub search: SearchRequestBuilder,
}

impl Model {
    pub fn new() -> Self {
        Self {
            embeddings: EmbeddingsRequestBuilder::new(),
            rerank: RerankRequestBuilder::new(),
            search: SearchRequestBuilder::new(),
        }
    }

    pub fn embeddings(&self) -> &EmbeddingsRequestBuilder {
        &self.embeddings
    }

    pub fn rerank(&self) -> &RerankRequestBuilder {
        &self.rerank
    }

    pub fn builder() -> ModelBuilder {
        ModelBuilder::new()
    }
}

pub struct ModelBuilder {
    embeddings: Option<EmbeddingsRequestBuilder>,
    rerank: Option<RerankRequestBuilder>,
}

impl ModelBuilder {
    pub fn new() -> Self {
        Self {
            embeddings: None,
            rerank: None,
        }
    }

    pub fn embeddings(mut self, embeddings: EmbeddingsRequestBuilder) -> Self {
        self.embeddings = Some(embeddings);
        self
    }

    pub fn rerank(mut self, rerank: RerankRequestBuilder) -> Self {
        self.rerank = Some(rerank);
        self
    }

    pub fn search(mut self, builder: SearchRequestBuilder) -> Self {
        self.search = Some(builder);
        self
    }

    pub fn build(self) -> Model {
        Model {
            embeddings: self
                .embeddings
                .unwrap_or_else(EmbeddingsRequestBuilder::new),
            rerank: self.rerank.unwrap_or_else(RerankRequestBuilder::new),
        }
    }
}

pub use crate::models::embeddings::EmbeddingModel as Embedding;
pub use crate::models::rerank::RerankModel as Rerank;
