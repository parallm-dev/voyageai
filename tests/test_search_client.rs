use std::sync::Arc;
use voyageai::client::{
    embeddings_client::EmbeddingClient, rerank_client::RerankClient, search_client::SearchClient,
};
use voyageai::config::VoyageConfig;

#[test]
fn test_search_client_creation() {
    let config = VoyageConfig::new("test_api_key".to_string());
    let rate_limiter = Arc::new(voyageai::client::RateLimiter::new());
    let embedding_client = EmbeddingClient::new(config.clone(), rate_limiter.clone());
    let rerank_client = RerankClient::new(config, rate_limiter);
    let _search_client = SearchClient::new(embedding_client, rerank_client);

    // We can't assert anything about private fields, so we just check that creation doesn't panic
}

// Note: We can't easily test the search functionality without mocking,
// which requires more complex setup. For now, we'll just test the client creation.
// In a real-world scenario, you might want to use integration tests with a real API
// or set up a more sophisticated mocking system.
