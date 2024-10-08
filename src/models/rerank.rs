use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RerankResult {
    pub object: Object,
    pub data: Vec<RerankData>,
    pub model: RerankModel,
    pub usage: Usage,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RerankData {
    pub object: Object,
    pub index: usize,
    pub relevance_score: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RerankModel {
    #[serde(rename = "voyage-rerank-v2")]
    VoyageRerankV2,
    #[serde(rename = "voyage-rerank-light-v2")]
    VoyageRerankLightV2,
}

impl RerankModel {
    pub const fn context_length(&self) -> usize {
        match self {
            Self::VoyageRerankV2 => 4096,
            Self::VoyageRerankLightV2 => 4096,
        }
    }

    pub const fn embedding_dimension(&self) -> usize {
        match self {
            Self::VoyageRerankV2 => 768,
            Self::VoyageRerankLightV2 => 384,
        }
    }
}

pub struct Object;
pub struct Usage;
