use voyageai::{RerankModel, RerankRequest, VoyageAiClient, VoyageConfig};
use voyageai::traits::llm::Embedder;

#[tokio::test]
async fn test_search_client_integration() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("VOYAGE_API_KEY").expect("VOYAGE_API_KEY must be set");
    let config = VoyageConfig::new(api_key);
    let client = VoyageAiClient::new_with_config(config);

    // Test direct embedding
    let text = "Test document";
    let embedding = client.embed_text(text).await.unwrap();
    assert!(!embedding[0].is_empty());

    // Test batch embedding
    let texts = vec!["test document 1".to_string(), "test document 2".to_string()];
    let embeddings = client.embed_batch(&texts).await.unwrap();
    assert_eq!(embeddings.len(), 2);
    assert!(!embeddings[0].is_empty());
    assert!(!embeddings[1].is_empty());

    // Test reranking
    let query = "What is Rust?";
    let documents = [
        "Rust is a systems programming language",
        "Python is interpreted",
    ];

    let rerank_request = RerankRequest::new(
        query.to_string(),
        documents.iter().map(|&s| s.to_string()).collect(),
        RerankModel::Rerank2,
        None,
    )
    .expect("Failed to create rerank request");

    let rerank_response = client.rerank(rerank_request).await?;
    assert!(!rerank_response.data.is_empty(), "Rerank response should not be empty");
    Ok(())
}
