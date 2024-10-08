use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

pub const BASE_URL: &str = "https://api.voyageai.com/v1";

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct RerankResponse {
    pub model: String,
    pub results: Vec<RerankResult>,
    pub usage: Usage,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RerankResult {
    pub index: usize,
    pub relevance_score: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub total_tokens: u32,
}

pub use crate::client::voyage_client::VoyageAiClient;

#[derive(Clone, Debug)]
pub struct RerankClient {
    client: Client,
    api_key: String,
}

impl RerankClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }

    pub fn builder() -> RerankClientBuilder {
        RerankClientBuilder::default()
    }

    pub async fn rerank<'a>(
        &self,
        request: RerankRequest<'a>,
    ) -> Result<RerankResponse, VoyageError> {
        let url = format!("{}/rerank", BASE_URL);

        let response = self
            .client
            .post(&url)
            .bearer_auth(&self.api_key)
            .json(&request)
            .send()
            .await?;

        if response.status().is_success() {
            let rerank_response = response.json::<RerankResponse>().await?;
            Ok(rerank_response)
        } else {
            Err(VoyageError::ApiError(response.text().await?))
        }
    }
}
#[derive(Default)]
pub struct RerankClientBuilder {
    api_key: Option<String>,
    client: Option<Client>,
}

impl RerankClientBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn api_key(mut self, api_key: String) -> Self {
        self.api_key = Some(api_key);
        self
    }

    pub fn client(mut self, client: Client) -> Self {
        self.client = Some(client);
        self
    }

    pub fn build(self) -> Result<RerankClient, VoyageError> {
        let api_key = self.api_key.ok_or(VoyageError::MissingApiKey)?;
        let client = self.client.unwrap_or_else(Client::new);

        Ok(RerankClient { client, api_key })
    }
}

#[async_trait]
pub trait Rerank {
    async fn rerank(&self, request: RerankRequest) -> Result<RerankResponse, VoyageError>;
}

#[async_trait]
impl Rerank for RerankClient {
    async fn rerank(&self, request: RerankRequest<'_>) -> Result<RerankResponse, VoyageError> {
        self.rerank(request).await
    }
}

impl std::fmt::Display for RerankResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Model: {}", self.model)?;
        writeln!(f, "Results:")?;
        for result in &self.results {
            writeln!(
                f,
                "  Index: {}, Score: {:.4}",
                result.index, result.relevance_score
            )?;
        }
        writeln!(f, "Prompt tokens: {}", self.usage.prompt_tokens)?;
        writeln!(f, "Total tokens: {}", self.usage.total_tokens)?;
        Ok(())
    }
}

impl RerankResponse {
    pub fn top_results(&self, n: usize) -> Vec<&RerankResult> {
        let mut sorted_results = self.results.iter().collect::<Vec<_>>();
        sorted_results.sort_by(|a, b| b.relevance_score.partial_cmp(&a.relevance_score).unwrap());
        sorted_results.truncate(n);
        sorted_results
    }
}

pub use crate::builder::RerankRequest;
pub use crate::errors::VoyageError;
