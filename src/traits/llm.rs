use crate::errors::VoyageError;
use crate::models::{
    embeddings::{EmbeddingModel, EmbeddingsInput, EmbeddingsRequest},
    rerank::{RerankModel, RerankRequest},
};
use crate::VoyageAiClient;
use async_trait::async_trait;

/// Interface for embedding text into vectors
#[async_trait]
pub trait Embedder: Send + Sync {
    /// Get embeddings for text
    async fn embed(&self, text: &str) -> Result<Vec<f32>, VoyageError>;

    /// Get embeddings for multiple texts
    async fn embed_batch(&self, texts: &[String]) -> Result<Vec<Vec<f32>>, VoyageError>;
}

/// Interface for reranking documents
#[async_trait]
pub trait Reranker: Send + Sync {
    /// Rerank documents based on a query
    async fn rerank(&self, query: &str, documents: Vec<String>) -> Result<Vec<f32>, VoyageError>;
}

impl VoyageAiClient {
    async fn embeddings(&self, request: EmbeddingsRequest) -> Result<Vec<Vec<f32>>, VoyageError> {
        let response = self.embeddings(request.input).await.map_err(|e| {
            VoyageError::ApiError(reqwest::StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;
        Ok(response.data.into_iter().map(|e| e.embedding).collect())
    }
}

#[async_trait]
impl Embedder for VoyageAiClient {
    async fn embed(&self, text: &str) -> Result<Vec<f32>, VoyageError> {
        let request = EmbeddingsRequest {
            input: EmbeddingsInput::Single(text.to_string()),
            model: EmbeddingModel::Voyage3,
            input_type: None,
            truncation: None,
            encoding_format: None,
        };

        let embeddings = self.embeddings(request).await?;
        Ok(embeddings[0].clone())
    }

    async fn embed_batch(&self, texts: &[String]) -> Result<Vec<Vec<f32>>, VoyageError> {
        let request = EmbeddingsRequest {
            input: EmbeddingsInput::Multiple(texts.to_vec()),
            model: EmbeddingModel::Voyage3,
            input_type: None,
            truncation: None,
            encoding_format: None,
        };

        let embeddings = self.embeddings(request).await?;
        Ok(embeddings)
    }
}
#[async_trait]
impl Reranker for VoyageAiClient {
    async fn rerank(&self, query: &str, documents: Vec<String>) -> Result<Vec<f32>, VoyageError> {
        let request = RerankRequest::new(query.to_string(), documents, RerankModel::Rerank2, None)
            .map_err(VoyageError::from)?;
        let response = self.rerank(request).await.map_err(|e| {
            VoyageError::ApiError(reqwest::StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;
        let scores: Vec<f32> = response
            .data
            .into_iter()
            .map(|r| r.relevance_score as f32)
            .collect();
        Ok(scores)
    }
}
