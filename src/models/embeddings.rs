use crate::VoyageError;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InputType {
    #[serde(rename = "query")]
    Query,
    #[serde(rename = "document")]
    Document,
}

#[derive(Debug, Serialize, Clone)]
#[serde(untagged)]
pub enum EmbeddingsInput {
    Single(String),
    Multiple(Vec<String>),
}

impl TryFrom<&[String]> for EmbeddingsInput {
    type Error = VoyageError;
    fn try_from(v: &[String]) -> Result<Self, Self::Error> {
        Ok(EmbeddingsInput::Multiple(v.to_vec()))
    }
}

impl TryFrom<Vec<&str>> for EmbeddingsInput {
    type Error = VoyageError;
    fn try_from(v: Vec<&str>) -> Result<Self, Self::Error> {
        Ok(EmbeddingsInput::Multiple(
            v.into_iter().map(String::from).collect(),
        ))
    }
}

impl From<&str> for EmbeddingsInput {
    fn from(s: &str) -> Self {
        EmbeddingsInput::Single(s.to_string())
    }
}

impl From<Vec<String>> for EmbeddingsInput {
    fn from(v: Vec<String>) -> Self {
        EmbeddingsInput::Multiple(v)
    }
}

#[derive(Debug, Serialize)]
pub struct EmbeddingsRequest {
    pub input: EmbeddingsInput,
    pub model: EmbeddingModel,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_type: Option<InputType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncation: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_format: Option<EncodingFormat>,
}

#[derive(Debug, Deserialize)]
pub struct EmbeddingsResponse {
    /// The type of object returned.
    #[serde(default)]
    pub object: String,
    /// A list of embedding data.
    pub data: Vec<EmbeddingData>,
    /// The model used for generating embeddings.
    #[serde(default)]
    pub model: String,
    /// Usage statistics for the request.
    pub usage: Usage,
}

/// Usage statistics for an embedding request.
#[derive(Debug, Deserialize)]
pub struct Usage {
    /// The total number of tokens used in the request.
    pub total_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbeddingData {
    pub object: String,
    pub embedding: Vec<f32>,
    pub index: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum EncodingFormat {
    #[serde(rename = "float")]
    Float,
    #[serde(rename = "base64")]
    Base64,
}

/// Supported embedding models by VoyageAI
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum EmbeddingModel {
    #[serde(rename = "voyage-3")]
    #[default]
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
