use voyageai::builder::embeddings::{EmbeddingsRequestBuilder, InputType};
use voyageai::models::embeddings::EmbeddingModel;
use voyageai::{VoyageAiClient, VoyageConfig};

#[tokio::test]
async fn test_embedding() {
    let api_key = std::env::var("VOYAGE_API_KEY").expect("VOYAGE_API_KEY must be set");
    let config = VoyageConfig::new(api_key);
    let client = VoyageAiClient::new(config);

    let inputs = vec![
        "This is the first test sentence.",
        "Here's another sentence to embed.",
        "And one final sentence for good measure.",
    ];

    let request = EmbeddingsRequestBuilder::new()
        .input_multiple(inputs.clone())
        .model(EmbeddingModel::Voyage3)
        .input_type(InputType::Document)
        .build()
        .expect("Failed to build embeddings request");

    let response = client
        .embeddings()
        .create_embedding(&request)
        .await
        .expect("Failed to create embeddings");

    assert_eq!(
        response.data.len(),
        inputs.len(),
        "Number of embeddings should match number of inputs"
    );

    for (i, embedding) in response.data.iter().enumerate() {
        assert_eq!(
            embedding.index, i,
            "Embedding index should match input index"
        );
        assert!(
            !embedding.embedding.is_empty(),
            "Embedding should not be empty"
        );
    }
}

#[tokio::test]
async fn test_embedding_single_input() {
    let api_key = std::env::var("VOYAGE_API_KEY").expect("VOYAGE_API_KEY must be set");
    let config = VoyageConfig::new(api_key);
    let client = VoyageAiClient::new(config);

    let input = "This is a single test sentence.";

    let request = EmbeddingsRequestBuilder::new()
        .input(input.to_string())
        .model(EmbeddingModel::Voyage3)
        .input_type(InputType::Document)
        .build()
        .expect("Failed to build embeddings request");

    let response = client
        .embeddings()
        .create_embedding(&request)
        .await
        .expect("Failed to create embedding");

    assert_eq!(response.data.len(), 1, "Should have a single embedding");
    assert!(
        !response.data[0].embedding.is_empty(),
        "Embedding should not be empty"
    );
}
