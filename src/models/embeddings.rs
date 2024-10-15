use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbeddingsResponse {
    pub object: String,
    pub data: Vec<EmbeddingData>,
    pub model: String,
    pub usage: Usage,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbeddingData {
    pub object: String,
    pub embedding: Vec<f32>,
    pub index: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Usage {
    pub total_tokens: u32,
}

/// Supported embedding models by VoyageAI
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmbeddingModel {
    #[serde(rename = "voyage-3")]
    Voyage3,
    #[serde(rename = "voyage-3-lite")]
    Voyage3Lite,
    #[serde(rename = "voyage-finance-2")]
    VoyageFinance2,
    #[serde(rename = "voyage-multilingual-2")]
    VoyageMultilingual2,
    #[serde(rename = "voyage-law-2")]
    VoyageLaw2,
}

impl EmbeddingModel {
    /// Returns the maximum context length for the model
    pub fn max_context_length(&self) -> usize {
        match self {
            Self::Voyage3 | Self::Voyage3Lite => 32000,
            Self::VoyageFinance2 | Self::VoyageMultilingual2 | Self::VoyageLaw2 => 16000,
        }
    }

    /// Returns the maximum number of tokens that can be processed in a single request
    pub fn max_tokens_per_request(&self) -> usize {
        match self {
            Self::Voyage3Lite => 1_000_000,
            Self::Voyage3 => 320_000,
            Self::VoyageFinance2 | Self::VoyageMultilingual2 | Self::VoyageLaw2 => 120_000,
        }
    }

    /// Returns the embedding dimension for the model
    pub fn embedding_dimension(&self) -> usize {
        match self {
            Self::Voyage3 => 512,
            Self::Voyage3Lite => 256,
            Self::VoyageFinance2 | Self::VoyageMultilingual2 | Self::VoyageLaw2 => 1024,
        }
    }
}

/// Input type for embeddings
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InputType {
    #[serde(rename = "query")]
    Query,
    #[serde(rename = "document")]
    Document,
}

/// Encoding format for embeddings
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EncodingFormat {
    #[serde(rename = "float")]
    Float,
    #[serde(rename = "base64")]
    Base64,
}
