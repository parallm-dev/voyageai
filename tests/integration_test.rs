use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;
use voyageai::builder::embeddings::{EmbeddingsRequestBuilder, InputType};
use voyageai::models::embeddings::EmbeddingModel;
use voyageai::models::rerank::{RerankModel, RerankRequest};
use voyageai::{VoyageAiClient, VoyageConfig};

#[tokio::test]
async fn test_voyage_ai_client() {
    // Set up logging
    let mut builder = Builder::from_default_env();
    builder
        .format(|buf, record| writeln!(buf, "{} - {}", record.level(), record.args()))
        .filter(None, LevelFilter::Debug)
        .init();

    // Initialize the client with the API key
    let api_key = std::env::var("VOYAGE_API_KEY").expect("VOYAGE_API_KEY must be set");
    let config = VoyageConfig::new(api_key);
    let client = VoyageAiClient::new(config);

    // Test embeddings
    let embeddings_request = EmbeddingsRequestBuilder::new()
        .input_multiple(vec![
            "Sample text 1".to_string(),
            "Sample text 2".to_string(),
        ])
        .model(EmbeddingModel::Voyage3)
        .input_type(InputType::Document)
        .build()
        .expect("Failed to build embeddings request");

    let embeddings_response = client
        .embeddings()
        .create_embedding(&embeddings_request)
        .await
        .expect("Failed to get embeddings");

    assert_eq!(embeddings_response.object, "list");
    assert_eq!(embeddings_response.data.len(), 2);
    assert_eq!(embeddings_response.model, "voyage-3");
    println!(
        "Embeddings tokens used: {}",
        embeddings_response.usage.total_tokens
    );

    for (i, embedding_data) in embeddings_response.data.iter().enumerate() {
        assert_eq!(embedding_data.object, "embedding");
        assert_eq!(embedding_data.index, i);
        assert!(!embedding_data.embedding.is_empty());
        println!("Embedding {} length: {}", i, embedding_data.embedding.len());
    }

    // Test rerank
    let query = "What is the capital of France?";
    let documents = vec![
        "Paris is the capital of France.",
        "London is the capital of the United Kingdom.",
        "Berlin is the capital of Germany.",
    ];

    let rerank_request = RerankRequest {
        query: query.to_string(),
        documents: documents.iter().map(|&s| s.to_string()).collect(),
        model: RerankModel::Rerank2,
        top_k: None,
    };

    let rerank_response = client
        .rerank()
        .rerank(&rerank_request)
        .await
        .expect("Failed to rerank documents");

    assert!(
        !rerank_response.results.is_empty(),
        "Rerank results are empty"
    );

    // Verify that the top result is relevant
    let top_result = &rerank_response.results[0];
    println!("Top document: {}", top_result.document);
    assert!(
        top_result.document.contains("Paris"),
        "Top document is not relevant"
    );

    // Print all rerank results for debugging
    println!("All rerank results:");
    for result in rerank_response.results.iter() {
        println!(
            "Result {}: Score: {}, Document: {}",
            result.index, result.relevance_score, result.document
        );
    }
    println!("Rerank model used: {}", rerank_response.model);
    println!("Rerank tokens used: {}", rerank_response.usage.total_tokens);
}
