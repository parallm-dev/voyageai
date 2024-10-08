use crate::builder::{EmbeddingsRequest, EmbeddingsRequestBuilder};
use crate::models::EmbeddingModel;
use crate::voyage_errors::VoyageError;
use serde::Deserialize;

const BASE_URL: &str = "https://api.voyageai.com/v1";

#[derive(Debug, Deserialize)]
pub struct EmbeddingsResponse {
    pub object: String,
    pub data: Vec<EmbeddingData>,
    pub model: EmbeddingModel,
    pub usage: Usage,
}

#[derive(Debug, Deserialize)]
pub struct EmbeddingData {
    pub object: String,
    pub embedding: Vec<f32>,
    pub index: usize,
}

#[derive(Debug, Deserialize)]
pub struct Usage {
    pub total_tokens: u32,
}

impl EmbeddingsRequest {
    pub async fn send(&self) -> Result<EmbeddingsResponse, VoyageError> {
        let client = reqwest::Client::new();
        let url = format!("{}/embeddings", BASE_URL);

        let response = client
            .post(&url)
            .bearer_auth(&self.voyage.api_key)
            .json(&self)
            .send()
            .await?;

        if response.status().is_success() {
            let embeddings_response = response.json::<EmbeddingsResponse>().await?;
            Ok(embeddings_response)
        } else {
            Err(VoyageError::ApiError(response.text().await?))
        }
    }
}

pub use builder::{EmbeddingsRequestBuilder, VoyageBuilder};
pub use client::embeddings::{EmbeddingData, EmbeddingsResponse, Usage};
pub use config::ClientConfig;
pub use errors::{VoyageBuilderError, VoyageError};
pub use models::EmbeddingModel;

pub struct VoyageAiClient {
    // Keep existing fields
}

impl VoyageAiClient {
    pub fn builder() -> VoyageBuilder {
        VoyageBuilder::new()
    }

    pub fn embeddings(&self) -> EmbeddingsRequestBuilder {
        EmbeddingsRequestBuilder::new().voyage(self.clone())
    }

    // Implement rerank method when ready
}
