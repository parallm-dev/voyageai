use crate::models::embeddings::{
    EmbeddingModel as EmbeddingModelType, EmbeddingsInput as EmbeddingsInputType,
    EmbeddingsRequest as EmbeddingsRequestType, InputType as InputTypeEnum, EncodingFormat,
};
use crate::errors::VoyageBuilderError;
use log::{debug, error};

#[derive(Debug, Default)]
pub struct EmbeddingsRequestBuilder {
    input: Option<EmbeddingsInputType>,
    model: Option<EmbeddingModelType>,
    input_type: Option<InputTypeEnum>,
    truncation: Option<bool>,
    encoding_format: Option<EncodingFormat>,
}

impl EmbeddingsRequestBuilder {
    pub fn new() -> Self {
        debug!("Creating new EmbeddingsRequestBuilder");
        Self::default()
    }

    pub fn input(mut self, input: impl Into<EmbeddingsInputType>) -> Self {
        debug!("Setting input for EmbeddingsRequestBuilder");
        self.input = Some(input.into());
        self
    }

    pub fn model(mut self, model: EmbeddingModelType) -> Self {
        debug!("Setting model for EmbeddingsRequestBuilder: {:?}", model);
        self.model = Some(model);
        self
    }

    pub fn input_type(mut self, input_type: InputTypeEnum) -> Self {
        debug!("Setting input_type for EmbeddingsRequestBuilder: {:?}", input_type);
        self.input_type = Some(input_type);
        self
    }

    pub fn truncation(mut self, truncation: bool) -> Self {
        debug!("Setting truncation for EmbeddingsRequestBuilder: {}", truncation);
        self.truncation = Some(truncation);
        self
    }

    pub fn encoding_format(mut self, encoding_format: EncodingFormat) -> Self {
        debug!("Setting encoding_format for EmbeddingsRequestBuilder: {:?}", encoding_format);
        self.encoding_format = Some(encoding_format);
        self
    }

    pub fn build(self) -> Result<EmbeddingsRequestType, VoyageBuilderError> {
        debug!("Building EmbeddingsRequest");
        let input = self.input.ok_or_else(|| {
            error!("Input is required for EmbeddingsRequest");
            VoyageBuilderError::MissingField("input".to_string())
        })?;
        let model = self.model.ok_or_else(|| {
            error!("Model is required for EmbeddingsRequest");
            VoyageBuilderError::MissingModel
        })?;

        Ok(EmbeddingsRequestType {
            input,
            model,
            input_type: self.input_type,
            truncation: self.truncation,
            encoding_format: self.encoding_format,
        })
    }
}
