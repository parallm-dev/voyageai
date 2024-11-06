use std::time::Duration;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum VoyageError {
    #[error("Missing documents: {0}")]
    MissingDocuments(String),

    #[error("Search builder error: {0}")]
    SearchBuilderError(String),

    #[error("Search index not built")]
    SearchIndexNotBuilt,

    #[error("Search dimension mismatch: expected {expected}, got {actual}")]
    SearchDimensionMismatch { expected: usize, actual: usize },

    #[error("Search result error: {0}")]
    SearchResultError(String),

    #[error("Bad Request (400): Invalid request format or parameters - {message}")]
    BadRequest { message: String },

    #[error("Unauthorized (401): Missing or invalid API key")]
    Unauthorized,

    #[error("Forbidden (403): {0}")]
    Forbidden(String),

    #[error("Not Found (404): {0}")]
    NotFound(String),

    #[error("Rate Limit Exceeded (429): Too many requests. Limit resets in {reset_in:?}")]
    RateLimitExceeded { reset_in: Duration },

    #[error("Internal Server Error (500): Unexpected server error - {message}")]
    InternalServerError { message: String },

    #[error("Service Unavailable (503)")]
    ServiceUnavailable,

    #[error("Missing API key")]
    MissingApiKey,

    #[error("JSON serialization/deserialization error: {0}")]
    JsonError(String),

    #[error("Tokenizer error: {0}")]
    TokenizerError(String),

    #[error("HTTP request error: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("Unhandled status code {0}: {1}")]
    UnhandledStatusCode(u16, String),

    #[error("API error (status {0}): {1}")]
    ApiError(reqwest::StatusCode, String),

    #[error("Input list too long: maximum of 128 texts allowed")]
    InputListTooLong,

    #[error("Total tokens exceed model limit: {0} tokens (limit: {1})")]
    TokenLimitExceeded(usize, usize),

    #[error("Document count exceeds limit: {0} documents (limit: 1000)")]
    TooManyDocuments(usize),

    #[error("Query and document token count exceeds limit: {0} tokens (limit: {1})")]
    QueryDocumentTokenLimitExceeded(usize, usize),

    #[error("Builder error: {0}")]
    BuilderError(String),

    #[error("No results found")]
    NoResults,
}

impl From<serde_json::Error> for VoyageError {
    fn from(error: serde_json::Error) -> Self {
        VoyageError::JsonError(error.to_string())
    }
}

use crate::models::rerank::ValidationError;

impl From<String> for VoyageError {
    fn from(message: String) -> Self {
        VoyageError::BuilderError(message)
    }
}

impl From<&str> for VoyageError {
    fn from(message: &str) -> Self {
        VoyageError::BuilderError(message.to_string())
    }
}

impl From<ValidationError> for VoyageError {
    fn from(err: ValidationError) -> Self {
        VoyageError::BuilderError(err.to_string())
    }
}

impl From<std::io::Error> for VoyageError {
    fn from(error: std::io::Error) -> Self {
        VoyageError::BuilderError(error.to_string())
    }
}

#[derive(Error, Debug)]
pub enum VoyageBuilderError {
    #[error("API key not set")]
    ApiKeyNotSet,

    #[error("Missing required field: {0}")]
    MissingField(String),

    #[error("Input list too long: maximum of 128 texts allowed")]
    InputListTooLong,

    #[error("Missing input")]
    MissingInput,

    #[error("Missing model")]
    MissingModel,

    #[error("Missing Voyage client")]
    MissingVoyage,
}

impl From<VoyageBuilderError> for VoyageError {
    fn from(error: VoyageBuilderError) -> Self {
        VoyageError::BuilderError(error.to_string())
    }
}
