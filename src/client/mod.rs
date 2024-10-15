pub mod client_limiter;
pub mod embeddings_client;
pub mod rerank_client;
pub mod search_client;
pub mod voyage_client;

pub use client_limiter::RateLimiter;
pub use embeddings_client::EmbeddingClient;
pub use rerank_client::RerankClient;
pub use search_client::{SearchClient, SearchRequest, SearchResponse, SearchResult, SearchType};
pub use voyage_client::VoyageAiClient;

// Re-export EmbeddingsResponse
pub use crate::models::embeddings::EmbeddingsResponse;
