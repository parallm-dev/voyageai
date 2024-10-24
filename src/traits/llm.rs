use crate::errors::VoyageError;
use crate::models::{EmbeddingModel, EmbeddingsInput, EmbeddingsRequest, embeddings::EmbeddingData};
use crate::VoyageAiClient;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct GenerateOptions {
    pub model: EmbeddingModel,
    pub max_tokens: usize,
    pub temperature: f32,
    pub top_p: f32,
    pub n: usize,
    pub stop: Option<Vec<String>>,
    pub presence_penalty: f32,
    pub frequency_penalty: f32,
    pub logit_bias: Option<HashMap<String, f32>>,
    pub user: Option<String>,
}

#[derive(Default)]
pub struct GenerateOptionsBuilder {
    model: Option<EmbeddingModel>,
    max_tokens: Option<usize>,
    temperature: Option<f32>,
    top_p: Option<f32>,
    n: Option<usize>,
    stop: Option<Vec<String>>,
    presence_penalty: Option<f32>,
    frequency_penalty: Option<f32>,
    logit_bias: Option<HashMap<String, f32>>,
    user: Option<String>,
}

impl GenerateOptionsBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn model(mut self, model: EmbeddingModel) -> Self {
        self.model = Some(model);
        self
    }

    pub fn max_tokens(mut self, max_tokens: usize) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    pub fn temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    pub fn top_p(mut self, top_p: f32) -> Self {
        self.top_p = Some(top_p);
        self
    }

    pub fn n(mut self, n: usize) -> Self {
        self.n = Some(n);
        self
    }

    pub fn stop(mut self, stop: Vec<String>) -> Self {
        self.stop = Some(stop);
        self
    }

    pub fn presence_penalty(mut self, presence_penalty: f32) -> Self {
        self.presence_penalty = Some(presence_penalty);
        self
    }

    pub fn frequency_penalty(mut self, frequency_penalty: f32) -> Self {
        self.frequency_penalty = Some(frequency_penalty);
        self
    }

    pub fn logit_bias(mut self, logit_bias: HashMap<String, f32>) -> Self {
        self.logit_bias = Some(logit_bias);
        self
    }

    pub fn user(mut self, user: String) -> Self {
        self.user = Some(user);
        self
    }

    pub fn build(self) -> Result<GenerateOptions, VoyageError> {
        let model = self.model.ok_or(VoyageError::BadRequest {
            message: "Model is required".to_string(),
        })?;

        Ok(GenerateOptions {
            model,
            max_tokens: self.max_tokens.unwrap_or(16),
            temperature: self.temperature.unwrap_or(1.0),
            top_p: self.top_p.unwrap_or(1.0),
            n: self.n.unwrap_or(1),
            stop: self.stop,
            presence_penalty: self.presence_penalty.unwrap_or(0.0),
            frequency_penalty: self.frequency_penalty.unwrap_or(0.0),
            logit_bias: self.logit_bias,
            user: self.user,
        })
    }
}

#[derive(Debug)]
pub struct GenerateResponse {
    pub text: String,
    pub finish_reason: String,
    pub usage: Usage,
    pub created: u64,
    pub model: String,
    pub choices: Vec<Choice>,
}

#[derive(Debug)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Debug)]
pub struct Choice {
    pub text: String,
    pub index: usize,
    pub logprobs: Option<LogProbs>,
    pub finish_reason: String,
}

#[derive(Debug)]
pub struct LogProbs {
    pub tokens: Vec<String>,
    pub token_logprobs: Vec<f32>,
    pub top_logprobs: Vec<HashMap<String, f32>>,
    pub text_offset: Vec<usize>,
}

pub async fn llm(
    client: &VoyageAiClient,
    prompt: &str,
    options: GenerateOptions,
) -> Result<GenerateResponse, VoyageError> {
    // Implement the API call to generate response
    // Placeholder implementation
    let _embedding_data = EmbeddingData {
        object: String::from("embedding"),
        embedding: vec![0.0; options.model.embedding_dimension()],
        index: 0,
    };

    let embeddings_request = EmbeddingsRequest {
        input: EmbeddingsInput::Single(prompt.to_string()),
        model: options.model,
        input_type: None,
        truncation: None,
        encoding_format: None,
    };

    let _response = client
        .embeddings()
        .create_embedding(&embeddings_request)
        .await?;
    // Process the response as needed
    Ok(GenerateResponse {
        text: format!("Generated response for prompt: {}", prompt),
        finish_reason: String::new(),
        usage: Usage {
            prompt_tokens: 0,
            completion_tokens: 0,
            total_tokens: 0,
        },
        created: 0,
        model: String::new(),
        choices: Vec::new(),
    })
}
