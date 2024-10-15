pub mod builder;
pub mod client;
pub mod config;
pub mod errors;
pub mod models;
pub mod traits;

pub use builder::{EmbeddingsRequestBuilder, RerankRequestBuilder, VoyageBuilder};
pub use client::voyage_client::VoyageAiClient;
pub use config::VoyageConfig;
pub use errors::{VoyageBuilderError, VoyageError};
pub use models::{
    embeddings::{EmbeddingModel, InputType},
    rerank::{RerankModel, RerankRequest, RerankResponse},
};

/// Re-export of the `tokio` crate for convenience.
pub use tokio;
