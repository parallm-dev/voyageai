use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RerankResponse {
    #[serde(default)]
    pub object: String,
    pub data: Vec<RerankResult>,
    #[serde(default)]
    pub model: String,
    pub usage: Usage,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RerankResult {
    pub relevance_score: f64,
    pub index: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RerankModel {
    #[serde(rename = "rerank-2")]
    Rerank2,
    #[serde(rename = "rerank-2-lite")]
    Rerank2Lite,
    #[serde(rename = "rerank-lite-1")]
    RerankLite1,
}

impl RerankModel {
    pub const fn max_context_length(&self) -> usize {
        match self {
            Self::Rerank2 => 16000,
            Self::Rerank2Lite | Self::RerankLite1 => 8000,
        }
    }

    pub const fn embedding_size(&self) -> usize {
        match self {
            Self::Rerank2 => 768,
            Self::Rerank2Lite | Self::RerankLite1 => 384,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Usage {
    pub total_tokens: u32,
}

#[derive(Debug, Serialize)]
pub struct RerankRequest {
    pub query: String,
    pub documents: Vec<String>,
    pub model: RerankModel,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<usize>,
}
