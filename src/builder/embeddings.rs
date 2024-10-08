use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Object {
    #[serde(rename = "list")]
    List,
    #[serde(rename = "embedding")]
    Embedding,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum EncodingFormat {
    #[serde(rename = "float")]
    Float,
    #[serde(rename = "base64")]
    Base64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbeddingsResponse {
    pub object: String,
    pub data: Vec<EmbeddingData>,
    pub model: EmbeddingModel,
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
use crate::errors::VoyageBuilderError as EmbeddingsBuilderError;
use crate::models::EmbeddingModel;
use crate::VoyageAiClient;

#[derive(Debug, Default)]
pub struct EmbeddingsRequestBuilder {
    input: Option<EmbeddingsInput>,
    model: Option<EmbeddingModel>,
    input_type: Option<InputType>,
    voyage: Option<VoyageAiClient>,
    truncation: Option<bool>,
    encoding_format: Option<EncodingFormat>,
}

impl EmbeddingsRequestBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn input(mut self, input: Vec<String>) -> Self {
        self.input = Some(EmbeddingsInput::Multiple(input));
        self
    }

    pub fn model(mut self, model: EmbeddingModel) -> Self {
        self.model = Some(model);
        self
    }

    pub fn input_type(mut self, input_type: InputType) -> Self {
        self.input_type = Some(input_type);
        self
    }

    pub fn voyage(mut self, voyage: VoyageAiClient) -> Self {
        self.voyage = Some(voyage);
        self
    }

    pub fn truncation(mut self, truncation: bool) -> Self {
        self.truncation = Some(truncation);
        self
    }

    pub fn encoding_format(mut self, encoding_format: EncodingFormat) -> Self {
        self.encoding_format = Some(encoding_format);
        self
    }

    pub fn build(self) -> Result<EmbeddingsRequest, EmbeddingsBuilderError> {
        let input = self.input.ok_or(EmbeddingsBuilderError::MissingInput)?;
        let model = self.model.ok_or(EmbeddingsBuilderError::MissingModel)?;
        let voyage = self.voyage.ok_or(EmbeddingsBuilderError::MissingVoyage)?;

        if let EmbeddingsInput::Multiple(ref texts) = input {
            if texts.len() > 128 {
                return Err(EmbeddingsBuilderError::InputListTooLong);
            }
        }

        Ok(EmbeddingsRequest {
            input,
            model,
            input_type: self.input_type,
            voyage,
            truncation: self.truncation,
            encoding_format: self.encoding_format,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputType {
    Query,
    Document,
}

#[derive(Debug)]
pub enum EmbeddingsInput {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Debug)]
pub struct EmbeddingsRequest {
    pub input: EmbeddingsInput,
    pub model: EmbeddingModel,
    pub input_type: Option<InputType>,
    pub voyage: VoyageAiClient,
    pub truncation: Option<bool>,
    pub encoding_format: Option<EncodingFormat>,
}

use crate::VoyageError;

impl EmbeddingsRequest {
    pub async fn send(self) -> Result<EmbeddingsResponse, VoyageError> {
        self.voyage.embeddings().create_embedding(&EmbeddingsRequest::from(self)).await
    }
}
