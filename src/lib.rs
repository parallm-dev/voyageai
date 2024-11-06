pub mod builder;
pub mod client;
pub mod config;
pub mod errors;
pub mod models;
pub mod traits;

pub use builder::{
    embeddings::EmbeddingsRequestBuilder, rerank::RerankRequestBuilder,
    search::SearchRequestBuilder, voyage::VoyageBuilder,
};
pub use client::voyage_client::VoyageAiClient;
pub use config::VoyageConfig;
pub use errors::{VoyageBuilderError, VoyageError};
pub use models::{
    embeddings::{EmbeddingModel, EmbeddingsInput, InputType},
    rerank::{RerankModel, RerankRequest, RerankResponse},
    search::{SearchModel, SearchType},
};

pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.is_empty() || b.is_empty() || a.len() != b.len() {
        return 0.0;
    }
    let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let magnitude_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let magnitude_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    if magnitude_a == 0.0 || magnitude_b == 0.0 {
        return 0.0;
    }
    dot_product / (magnitude_a * magnitude_b)
}
