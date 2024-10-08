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

impl RerankRequest {
    pub async fn send(self) -> Result<RerankResponse, VoyageError> {
        let client = reqwest::Client::new();
        let url = format!("{}/rerank", BASE_URL);

        let response = client
            .post(&url)
            .bearer_auth(&self.voyage.api_key)
            .json(&self)
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

pub use crate::client::RerankRequest;
pub use crate::errors::VoyageError;
