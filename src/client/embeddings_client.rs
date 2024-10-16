use crate::models::embeddings::{EmbeddingsRequest, EmbeddingsResponse};
use crate::client::RateLimiter;
use crate::config::VoyageConfig;
use crate::errors::VoyageError;
use log::{debug, info, warn};
use reqwest::Client;
use serde::Deserialize;
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
            .json(request)
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        if status.is_success() {
            debug!("Embedding request successful");
            let embeddings_response: EmbeddingsResponse = serde_json::from_str(&text)?;

            self.rate_limiter
                .update_embeddings_usage(embeddings_response.usage.total_tokens)
                .await;

            Ok(embeddings_response)
        } else {
            warn!("Embedding request failed with status: {}", status);
            Err(VoyageError::ApiError(status, text))
        }
    }

    /// Estimates the number of tokens in the request.
    fn estimate_tokens(&self, _request: &EmbeddingsRequest) -> u32 {
        // Implement token estimation logic here
        // For now, we'll return a placeholder value
        100
    }
}

/// Represents a single embedding in the response.
#[derive(Debug, Deserialize)]
pub struct EmbeddingData {
    /// The type of object.
    pub object: String,
    /// The embedding vector.
    pub embedding: Vec<f32>,
    /// The index of this embedding in the request.
    pub index: usize,
}
