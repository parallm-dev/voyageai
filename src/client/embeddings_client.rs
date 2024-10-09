use crate::builder::EmbeddingsRequest;
use crate::config::VoyageConfig;
use crate::errors::VoyageError;
use crate::models::EmbeddingModel;
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

    pub async fn create_embedding(
        &self,
        request: &EmbeddingsRequest,
    ) -> Result<EmbeddingsResponse, VoyageError> {
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

    pub fn input<T: Into<String>>(
        &self,
        input: T,
    ) -> crate::builder::EmbeddingsRequestBuilder<VoyageConfig> {
        crate::builder::EmbeddingsRequestBuilder::<VoyageConfig>::new().input(input)
    }

    pub fn input_multiple<I, T>(
        &self,
        input: I,
    ) -> crate::builder::EmbeddingsRequestBuilder<VoyageConfig>
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        crate::builder::EmbeddingsRequestBuilder::<VoyageConfig>::new().input_multiple(input)
    }
}

#[derive(Debug, Deserialize)]
pub struct EmbeddingData {
    pub object: String,
    pub embedding: Vec<f32>,
    pub index: usize,
}
