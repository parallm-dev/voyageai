use reqwest::Client;
use serde::Deserialize;
use crate::builder::EmbeddingsRequest;
use crate::config::VoyageConfig;
use crate::errors::VoyageError;
use crate::models::EmbeddingModel;

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

impl Default for EmbeddingClient {
    fn default() -> Self {
        Self::new(VoyageConfig::default())
    }
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

    pub fn input(&self, input: impl Into<String>) -> crate::builder::EmbeddingsRequestBuilder {
        crate::builder::EmbeddingsRequestBuilder::new().input(input)
    }

    pub fn input_multiple(&self, input: impl IntoIterator<Item = impl Into<String>>) -> crate::builder::EmbeddingsRequestBuilder {
        crate::builder::EmbeddingsRequestBuilder::new().input_multiple(input)
    }

    pub fn input_multiple(&self, input: Vec<String>) -> crate::builder::EmbeddingsRequestBuilder {
        crate::builder::EmbeddingsRequestBuilder::new().input(input)
    }
}

#[derive(Debug, Deserialize)]
pub struct EmbeddingData {
    pub object: String,
    pub embedding: Vec<f32>,
    pub index: usize,
}
