pub mod builder;
pub mod client;
pub mod config;
pub mod models;

pub use builder::{EmbeddingsRequestBuilder, RerankRequestBuilder, VoyageBuilder};
pub use client::embeddings_client::{EmbeddingData, EmbeddingsResponse, EmbeddingsUsage};
pub use client::rerank_client::{RerankResponse, RerankResult, RerankUsage};
pub use client::voyage_client::VoyageAiClient;
pub use config::ClientConfig;
pub use models::{EmbeddingModel, RerankModel};

pub use builder::voyage;
