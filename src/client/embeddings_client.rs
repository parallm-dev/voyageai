pub use crate::errors::VoyageError;
pub use crate::models::EmbeddingData;
pub use crate::models::EmbeddingModel;
pub use crate::models::EmbeddingsResult;

pub use crate::config::VoyageConfig;
pub use crate::errors::VoyageBuilderError;

use reqwest::Client;
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

#[derive(Debug, Clone)]
pub struct EmbeddingClient {
    client: Client,
    config: VoyageConfig,
}

pub type EmbeddingsClient = EmbeddingClient;

pub struct EmbeddingClient {
    client: Client,
    config: VoyageConfig,
}

impl EmbeddingClient {
    pub fn new(config: VoyageConfig) -> Self {
        Self {
            client: Client::new(),
            config,
        }
    }

    pub async fn create_embedding(&self, request: &EmbeddingsRequest) -> Result<EmbeddingsResponse, VoyageError> {
        let url = format!("{}/embeddings", BASE_URL);

        let response = self
            .client
            .post(&url)
            .bearer_auth(&self.config.api_key)
            .json(request)
            .send()
            .await?;

        if response.status().is_success() {
            let embeddings_response = response.json::<EmbeddingsResponse>().await?;
            Ok(embeddings_response)
        } else {
            Err(VoyageError::ApiError(response.text().await?))
        }
    }
    pub fn new(config: VoyageConfig) -> Self {
        Self {
            client: Client::new(),
            config,
        }
    }

    pub fn input(&self, input: impl Into<String>) -> crate::builder::EmbeddingsRequestBuilder {
        let input_string: String = input.into();
        crate::builder::EmbeddingsRequestBuilder::new().input(vec![input_string])
    }

    pub fn input_multiple(&self, input: Vec<String>) -> crate::builder::EmbeddingsRequestBuilder {
        crate::builder::EmbeddingsRequestBuilder::new().input(input)
    }

impl EmbeddingClient {
    pub async fn create_embedding(
        &self,
        request: &EmbeddingsRequest,
    ) -> Result<EmbeddingsResponse, VoyageError> {
        let url = format!("{}/embeddings", BASE_URL);

        let response = self
            .client
            .post(&url)
            .bearer_auth(&self.config.api_key)
            .json(&request)
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
}
