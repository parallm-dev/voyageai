use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RerankResponse {
    pub object: String,
    pub rankings: Vec<Ranking>,
    pub model: RerankModel,
    pub usage: Usage,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ranking {
    pub index: usize,
    pub score: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RerankModel {
    #[serde(rename = "voyage-rerank-v2")]
    V2,
    #[serde(rename = "voyage-rerank-light-v2")]
    LightV2,
}

impl RerankModel {
    pub const fn max_context_length(&self) -> usize {
        match self {
            Self::V2 | Self::LightV2 => 4096,
        }
    }

    pub const fn embedding_size(&self) -> usize {
        match self {
            Self::V2 => 768,
            Self::LightV2 => 384,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Usage {
    pub total_tokens: u32,
}
