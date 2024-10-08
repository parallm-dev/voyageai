use serde::{Deserialize, Serialize};
use crate::errors::VoyageBuilderError as EmbeddingsBuilderError;
use crate::models::EmbeddingModel;
use crate::VoyageAiClient;
use crate::VoyageError;

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

#[derive(Debug)]
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
        Self {
            input: None,
            model: None,
            input_type: None,
            voyage: None,
            truncation: None,
            encoding_format: None,
        }
    }

    pub fn with_client(mut self, voyage: impl Into<VoyageAiClient>) -> Self {
        self.voyage = Some(voyage.into());
        self
    }

    pub fn input<T: Into<EmbeddingsInput>>(mut self, input: T) -> Self {
        self.input = Some(input.into());
        self
    }

    pub fn input_multiple<I, S>(mut self, input: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.input = Some(EmbeddingsInput::Multiple(input.into_iter().map(Into::into).collect()));
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

        match &input {
            EmbeddingsInput::Single(_) => {},
            EmbeddingsInput::Multiple(texts) => {
                if texts.len() > 128 {
                    return Err(EmbeddingsBuilderError::InputListTooLong);
                }
            }
        }

        Ok(EmbeddingsRequest {
            input: match input {
                EmbeddingsInput::Single(s) => vec![s],
                EmbeddingsInput::Multiple(v) => v,
            },
            model,
            encoding_format: self.encoding_format,
            user: None,
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

impl<T: Into<String>> From<T> for EmbeddingsInput {
    fn from(s: T) -> Self {
        EmbeddingsInput::Single(s.into())
    }
}

impl<T: Into<String>> FromIterator<T> for EmbeddingsInput {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        EmbeddingsInput::Multiple(iter.into_iter().map(Into::into).collect())
    }
}

#[derive(Debug, Serialize)]
pub struct EmbeddingsRequest {
    pub input: Vec<String>,
    pub model: EmbeddingModel,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_format: Option<EncodingFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl EmbeddingsRequest {
    pub async fn send(self, client: &VoyageAiClient) -> Result<crate::client::embeddings_client::EmbeddingsResponse, VoyageError> {
        client.embeddings().create_embedding(&self).await
    }
}
