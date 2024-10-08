pub use crate::errors::VoyageError;
pub use crate::models::EmbeddingData;
pub use crate::models::EmbeddingModel;
pub use crate::models::EmbeddingsResult;

pub use crate::config::VoyageConfig;
pub use crate::errors::VoyageBuilderError;

use serde::Deserialize;

pub const BASE_URL: &str = "https://api.voyageai.com/v1";

#[derive(Debug, Deserialize)]
pub struct EmbeddingsResponse {
    pub object: String,
    pub data: Vec<EmbeddingData>,
    pub model: EmbeddingModel,
    pub usage: Usage,
}

#[derive(Debug, Deserialize)]
pub struct Usage {
    pub total_tokens: u32,
}

impl EmbeddingsResult {
    pub async fn send(&self, config: &VoyageConfig) -> Result<EmbeddingsResponse, VoyageError> {
        let client = reqwest::Client::new();
        let url = format!("{}/embeddings", BASE_URL);

        let response = client
            .post(&url)
            .bearer_auth(&config.api_key)
            .json(&self.data)
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
