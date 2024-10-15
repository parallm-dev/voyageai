use crate::models::embeddings::{
    EmbeddingModel as EmbeddingModelType, EmbeddingsInput as EmbeddingsInputType,
    EmbeddingsRequest as EmbeddingsRequestType, InputType as InputTypeEnum,
};

pub struct EmbeddingsRequestBuilder {
    input: Option<EmbeddingsInputType>,
    model: Option<EmbeddingModelType>,
    input_type: Option<InputTypeEnum>,
    truncation: Option<bool>,
}

impl Default for EmbeddingsRequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl EmbeddingsRequestBuilder {
    pub fn new() -> Self {
        Self {
            input: None,
            model: None,
            input_type: None,
            truncation: None,
        }
    }

    pub fn input(mut self, input: impl Into<EmbeddingsInputType>) -> Self {
        self.input = Some(input.into());
        self
    }

    pub fn model(mut self, model: EmbeddingModelType) -> Self {
        self.model = Some(model);
        self
    }

    pub fn input_type(mut self, input_type: InputTypeEnum) -> Self {
        self.input_type = Some(input_type);
        self
    }

    pub fn truncation(mut self, truncation: bool) -> Self {
        self.truncation = Some(truncation);
        self
    }

    pub fn build(self) -> Result<EmbeddingsRequestType, &'static str> {
        let input = self.input.ok_or("Input is required")?;
        let model = self.model.ok_or("Model is required")?;

        Ok(EmbeddingsRequestType {
            input,
            model,
            input_type: self.input_type,
            truncation: self.truncation,
            encoding_format: None,
        })
    }
}
