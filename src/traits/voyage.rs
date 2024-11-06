use crate::models::embeddings::{EmbeddingsInput, EmbeddingsResponse};
use crate::models::rerank::{RerankRequest, RerankResponse};
use crate::client::SearchRequest;
use crate::client::SearchResult;
use async_trait::async_trait;

#[async_trait]
pub trait VoyageAiClientExt {
    async fn embed(
        &self,
        input: impl Into<EmbeddingsInput> + Send,
    ) -> Result<EmbeddingsResponse, Box<dyn std::error::Error>>;

    async fn rerank(
        &self,
        request: RerankRequest,
    ) -> Result<RerankResponse, Box<dyn std::error::Error>>;

    async fn search(
        &self,
        request: SearchRequest,
    ) -> Result<Vec<SearchResult>, Box<dyn std::error::Error>>;
}
