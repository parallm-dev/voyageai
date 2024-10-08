use thiserror::Error;

#[derive(Debug, Error)]
pub enum VoyageError {
    #[error("Invalid request: {message}")]
    InvalidRequest { message: String },
    #[error("Unauthorized: Invalid API key")]
    Unauthorized,
    #[error("Rate limit exceeded")]
    RateLimitExceeded,
    #[error("Missing API key")]
    MissingApiKey,
    #[error("Server error: {0}")]
    ServerError(String),
    #[error("Service unavailable")]
    ServiceUnavailable,
    #[error("JSON serialization/deserialization error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("Tokenizer error: {0}")]
    TokenizerError(String),
    #[error("HTTP request error: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Forbidden: {0}")]
    Forbidden(String),
    #[error("Not Found: {0}")]
    NotFound(String),
    #[error("Unhandled status code {0}: {1}")]
    UnhandledStatusCode(u16, String),
    #[error("API error: {0}")]
    ApiError(String),
    #[error("Rerank error: Invalid input")]
    RerankInvalidInput,
    #[error("Rerank error: Missing query")]
    RerankMissingQuery,
    #[error("Rerank error: Missing documents")]
    RerankMissingDocuments,
}

#[derive(Error, Debug)]
pub enum VoyageBuilderError {
    #[error("API key not set")]
    ApiKeyNotSet,
    #[error("Missing required field: {0}")]
    MissingField(String),
    #[error("Input list too long for rerank")]
    InputListTooLong,
    #[error("Missing input for rerank")]
    MissingInput,
    #[error("Missing model for rerank")]
    MissingModel,
    #[error("Missing Voyage client for rerank")]
    MissingVoyage,
}
