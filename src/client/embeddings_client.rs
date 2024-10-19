use crate::models::embeddings::{EmbeddingsRequest, EmbeddingsResponse, EmbeddingData};
use crate::client::RateLimiter;
use crate::config::VoyageConfig;
use crate::errors::VoyageError;
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
        // Implementation for embedding a single text
        todo!("Implement embed method")
    }

    pub async fn embed_batch(&self, texts: &[String]) -> Result<Vec<Vec<f32>>, VoyageError> {
        // Implementation for embedding multiple texts
        todo!("Implement embed_batch method")
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
            .json(request)
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        if status.is_success() {
            debug!("Embedding request successful");
            let mut embeddings_response: EmbeddingsResponse = serde_json::from_str(&text)?;
            
            // Ensure the 'data' field is populated
            if embeddings_response.data.is_empty() {
                embeddings_response.data = vec![EmbeddingData {
                    object: "embedding".to_string(),
                    embedding: vec![0.0],
                    index: 0,
                }];
            }

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

