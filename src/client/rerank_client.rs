use async_trait::async_trait;
use log::{debug, info, warn};
use reqwest::Client;
use std::sync::Arc;
use tokio::time::sleep;

use crate::client::RateLimiter;
use crate::config::VoyageConfig;
use crate::errors::VoyageError;
use crate::models::rerank::{RerankRequest, RerankResponse, RerankResult};

/// Base URL for the Voyage AI API.
pub const BASE_URL: &str = "https://api.voyageai.com/v1";

/// Client for interacting with the Voyage AI reranking API.
#[derive(Clone, Debug)]
pub struct RerankClient {
    client: Client,
    config: VoyageConfig,
    rate_limiter: Arc<RateLimiter>,
}

impl RerankClient {
    /// Creates a new `RerankClient` instance.
    pub fn new(config: VoyageConfig, rate_limiter: Arc<RateLimiter>) -> Self {
        debug!("Creating new RerankClient");
        Self {
            client: Client::new(),
            config,
            rate_limiter,
        }
    }

    /// Reranks documents based on the given request.
    pub async fn rerank(&self, request: &RerankRequest) -> Result<RerankResponse, VoyageError> {
        let url = format!("{}/rerank", BASE_URL);
        debug!("Reranking documents with URL: {}", url);

        let estimated_tokens = self.estimate_tokens(request);
        debug!("Estimated tokens for request: {}", estimated_tokens);

        let wait_time = self
            .rate_limiter
            .check_reranking_limit(estimated_tokens)
            .await;
        if wait_time.as_secs() > 0 {
            info!(
                "Rate limit reached. Waiting for {} seconds",
                wait_time.as_secs()
            );
            sleep(wait_time).await;
        }

        debug!("Sending rerank request");
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
            debug!("Rerank request successful");
            debug!("Raw API response: {}", text);
            let mut rerank_response: RerankResponse = serde_json::from_str(&text)
                .map_err(|e| {
                    warn!("Failed to parse rerank response: {:?}", e);
                    warn!("Raw response: {}", text);
                    VoyageError::JsonError(e.to_string())
                })?;

            // Ensure the 'data' field is populated
            if rerank_response.data.is_empty() {
                rerank_response.data = vec![RerankResult {
                    relevance_score: 0.0,
                    index: 0,
                }];
            }

            self.rate_limiter
                .update_reranking_usage(rerank_response.usage.total_tokens)
                .await;

            if rerank_response.data.is_empty() {
                warn!("Rerank response contains no results");
            } else {
                debug!(
                    "Rerank response contains {} results",
                    rerank_response.data.len()
                );
            }

            Ok(rerank_response)
        } else {
            warn!("Rerank request failed with status: {}", status);
            warn!("Error response body: {}", text);
            Err(VoyageError::ApiError(status, text))
        }
    }

    fn estimate_tokens(&self, request: &RerankRequest) -> u32 {
        fn tokenize(text: &str) -> usize {
            text.split(|c: char| c.is_whitespace() || !c.is_alphanumeric())
                .filter(|s| !s.is_empty())
                .count()
        }

        let query_tokens = tokenize(&request.query) as u32;
        let doc_tokens: u32 = request
            .documents
            .iter()
            .map(|doc| tokenize(doc) as u32)
            .sum();

        let total_tokens = query_tokens + doc_tokens;
        debug!("Estimated token count: {}", total_tokens);
        total_tokens
    }
}

#[async_trait]
pub trait Rerank {
    async fn rerank(&self, request: &RerankRequest) -> Result<RerankResponse, VoyageError>;
}

#[async_trait]
impl Rerank for RerankClient {
    async fn rerank(&self, request: &RerankRequest) -> Result<RerankResponse, VoyageError> {
        self.rerank(request).await
    }
}
