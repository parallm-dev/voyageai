use async_trait::async_trait;
use log::{debug, info, warn};
use reqwest::Client;
use std::sync::Arc;
use tokio::time::sleep;

use crate::client::RateLimiter;
use crate::config::VoyageConfig;
use crate::errors::VoyageError;
use crate::models::rerank::{RerankRequest, RerankResponse};

/// Base URL for the Voyage AI API.
const BASE_URL: &str = "https://api.voyageai.com/v1";

/// Client trait for interacting with the Voyage AI reranking API.
#[async_trait]
pub trait RerankClient: std::fmt::Debug + Send + Sync {
    /// Reranks documents based on the given request.
    async fn rerank(&self, request: &RerankRequest) -> Result<RerankResponse, VoyageError>;
}

/// Default implementation of RerankClient
#[derive(Clone, Debug)]
pub struct DefaultRerankClient {
    client: Client,
    config: VoyageConfig,
    rate_limiter: Arc<RateLimiter>,
}

impl DefaultRerankClient {
    /// Creates a new `DefaultRerankClient` instance.
    pub fn new(config: VoyageConfig, rate_limiter: Arc<RateLimiter>) -> Self {
        debug!("Creating new DefaultRerankClient");
        Self {
            client: Client::new(),
            config,
            rate_limiter,
        }
    }

    fn estimate_tokens(&self, request: &RerankRequest) -> u32 {
        fn tokenize(text: &str) -> usize {
            text.split(|c: char| c.is_whitespace() || !c.is_alphanumeric())
                .filter(|s| !s.is_empty())
                .count()
        }

        let query_tokens = tokenize(&request.query);
        let doc_tokens: usize = request.documents.iter().map(|doc| tokenize(doc)).sum();

        let total_tokens = query_tokens + doc_tokens;
        debug!("Estimated token count: {}", total_tokens);
        total_tokens as u32
    }
}

#[async_trait]
impl RerankClient for DefaultRerankClient {
    async fn rerank(&self, request: &RerankRequest) -> Result<RerankResponse, VoyageError> {
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

        let response = self
            .client
            .post(&url)
            .bearer_auth(self.config.api_key())
            .json(request)
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        

        match status {
            reqwest::StatusCode::OK => {
                debug!("Rerank request successful");
                debug!("Raw API response: {}", text);
                let rerank_response: RerankResponse = serde_json::from_str(&text).map_err(|e| {
                    warn!("Failed to parse rerank response: {:?}", e);
                    warn!("Raw response: {}", text);
                    VoyageError::JsonError(e.to_string())
                })?;

                if rerank_response.data.is_empty() {
                    warn!("Rerank response contains no results");
                } else {
                    debug!(
                        "Rerank response contains {} results",
                        rerank_response.data.len()
                    );
                }

                self.rate_limiter
                    .update_reranking_usage(rerank_response.usage.total_tokens)
                    .await;

                Ok(rerank_response)
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                warn!("Unauthorized request: {}", text);
                Err(VoyageError::Unauthorized)
            }
            _ => {
                warn!("Rerank request failed with status: {}", status);
                warn!("Error response body: {}", text);
                Err(VoyageError::ApiError(status, text))
            }
        }
    }
}
