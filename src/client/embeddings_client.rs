use crate::client::RateLimiter;
use crate::config::VoyageConfig;
use crate::models::embeddings::{
    EmbeddingData, EmbeddingsInput, EmbeddingsRequest, EmbeddingsResponse,
};
use crate::VoyageError;

use log::{debug, info, warn};
use reqwest::Client;
use std::sync::Arc;
use tokio::time::sleep;

/// Base URL for the Voyage AI API.
pub const BASE_URL: &str = "https://api.voyageai.com/v1";

/// Client for interacting with the Voyage AI embeddings API.
#[derive(Debug, Clone)]
pub struct EmbeddingClient {
    client: Client,
    config: VoyageConfig,
    rate_limiter: Arc<RateLimiter>,
}

impl EmbeddingClient {
    pub async fn embed(&self, text: &str) -> Result<Vec<f32>, VoyageError> {
        let request = EmbeddingsRequest {
            input: EmbeddingsInput::Single(text.to_string()),
            model: self.config.embedding_model,
            input_type: None,
            truncation: None,
            encoding_format: None,
        };
        self.create_embedding(&request)
            .await
            .map(|response| response.data[0].embedding.clone())
    }

    pub async fn embed_batch(&self, texts: &[String]) -> Result<Vec<Vec<f32>>, VoyageError> {
        if texts.is_empty() {
            return Ok(Vec::new());
        }
        let request = EmbeddingsRequest {
            input: EmbeddingsInput::Multiple(texts.to_vec()),
            model: self.config.embedding_model,
            input_type: None,
            truncation: None,
            encoding_format: None,
        };
        self.create_embedding(&request)
            .await
            .map(|response| response.data.into_iter().map(|d| d.embedding).collect())
    }
}

impl EmbeddingClient {
    /// Creates a new `EmbeddingClient` instance.
    pub fn new(config: VoyageConfig, rate_limiter: Arc<RateLimiter>) -> Self {
        debug!("Creating new EmbeddingClient");
        Self {
            client: Client::new(),
            config,
            rate_limiter,
        }
    }

    /// Creates embeddings for the given request.
    pub async fn create_embedding(
        &self,
        request: &EmbeddingsRequest,
    ) -> Result<EmbeddingsResponse, VoyageError> {
        let url = format!("{}/embeddings", BASE_URL);
        debug!("Creating embedding with URL: {}", url);

        let estimated_tokens = self.estimate_tokens(request);
        debug!("Estimated tokens for request: {}", estimated_tokens);

        let wait_time = self
            .rate_limiter
            .check_embeddings_limit(estimated_tokens)
            .await;
        if wait_time.as_secs() > 0 {
            info!(
                "Rate limit reached. Waiting for {} seconds",
                wait_time.as_secs()
            );
            sleep(wait_time).await;
        }

        debug!("Sending embedding request");
        let response = self
            .client
            .post(&url)
            .bearer_auth(self.config.api_key())
            .json(&request)
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        

        match status {
            reqwest::StatusCode::OK => {
                debug!("Embedding request successful");
                let embeddings_response: EmbeddingsResponse = serde_json::from_str(&text)?;

                let embeddings_response = if embeddings_response.data.is_empty() {
                    EmbeddingsResponse {
                        data: vec![EmbeddingData {
                            object: "embedding".to_string(),
                            embedding: vec![0.0],
                            index: 0,
                        }],
                        ..embeddings_response
                    }
                } else {
                    embeddings_response
                };

                self.rate_limiter
                    .update_embeddings_usage(embeddings_response.usage.total_tokens)
                    .await;

                Ok(embeddings_response)
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                warn!("Unauthorized: Invalid API key");
                Err(VoyageError::Unauthorized)
            }
            reqwest::StatusCode::FORBIDDEN => {
                warn!("Forbidden: {}", text);
                Err(VoyageError::Forbidden(text))
            }
            _ => {
                warn!("Embedding request failed with status: {}", status);
                Err(VoyageError::ApiError(status, text))
            }
        }
    }

    /// Estimates the number of tokens in the request by approximating based on the input text length.
    fn estimate_tokens(&self, request: &EmbeddingsRequest) -> u32 {
        match &request.input {
            EmbeddingsInput::Single(text) => {
                // Rough estimate: 1 token per 4 characters + small overhead
                let base_tokens = (text.len() as f32 / 4.0).ceil() as u32;
                base_tokens + 2
            }
            EmbeddingsInput::Multiple(texts) => {
                // Calculate tokens for each text and sum
                let mut total = 0;
                for text in texts {
                    total += (text.len() as f32 / 4.0).ceil() as u32;
                }
                // Add overhead for batch processing
                total + (2 * texts.len() as u32)
            }
        }
    }
}
