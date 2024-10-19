pub mod builder {
    pub mod embeddings;
    pub mod voyage;
    pub mod search;
}
pub mod client;
pub mod config;
pub mod errors;
pub mod models;

pub mod traits;

pub use crate::builder::embeddings::EmbeddingsRequestBuilder;
pub use crate::builder::voyage::VoyageBuilder;
pub use client::voyage_client::VoyageAiClient;
pub use config::VoyageConfig;
pub use errors::{VoyageBuilderError, VoyageError};
pub use models::{
    embeddings::{EmbeddingModel, InputType},
    rerank::{RerankModel, RerankRequest, RerankResponse},
    search::{SearchModel, SearchType},
};

/// Re-export of the `tokio` crate for convenience.
pub use tokio;
