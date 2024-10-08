pub struct EmbeddingsResult {
    pub object: Object,
    pub data: Vec<EmbeddingData>,
    pub model: EmbeddingModel,
    pub usage: Usage,
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmbeddingModel {
    #[serde(rename = "voyage-large-2-instruct")]
    VoyageLarge2Instruct,
    #[serde(rename = "voyage-finance-2")]
    VoyageFinance2,
    #[serde(rename = "voyage-multilingual-2")]
    VoyageMultilingual2,
    #[serde(rename = "voyage-law-2")]
    VoyageLaw2,
    #[serde(rename = "voyage-code-2")]
    VoyageCode2,
    #[serde(rename = "voyage-large-2")]
    VoyageLarge2,
    #[serde(rename = "voyage-2")]
    Voyage2,
    #[serde(rename = "voyage-3")]
    Voyage3,
    #[serde(rename = "voyage-3-lite")]
    Voyage3Lite,
}

impl EmbeddingModel {
    pub const fn context_length(&self) -> usize {
        match self {
            Self::VoyageLarge2Instruct => 16000,
            Self::VoyageFinance2 => 32000,
            Self::VoyageMultilingual2 => 32000,
            Self::VoyageLaw2 => 16000,
            Self::VoyageCode2 => 16000,
            Self::VoyageLarge2 => 16000,
            Self::Voyage2 => 4000,
            Self::Voyage3 => 32000,
            Self::Voyage3Lite => 32000,
        }
    }

    pub const fn embedding_dimension(&self) -> usize {
        match self {
            Self::VoyageLarge2Instruct => 1024,
            Self::VoyageFinance2 => 1024,
            Self::VoyageMultilingual2 => 1024,
            Self::VoyageLaw2 => 1024,
            Self::VoyageCode2 => 1536,
            Self::VoyageLarge2 => 1536,
            Self::Voyage2 => 1024,
            Self::Voyage3 => 512,
            Self::Voyage3Lite => 256,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbeddingData {
    pub object: Object,
    pub embedding: Vec<f32>,
    pub index: usize,
}
