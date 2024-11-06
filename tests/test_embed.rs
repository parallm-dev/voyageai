use voyageai::EmbeddingsRequestBuilder;
use voyageai::{EmbeddingModel, EmbeddingsInput};
use voyageai::{InputType, VoyageAiClient, VoyageConfig};

#[tokio::test]
async fn test_embedding() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("VOYAGE_API_KEY").unwrap_or_else(|_| "test_key".to_string());
    let config = VoyageConfig::new(api_key);
    let client = VoyageAiClient::new_with_config(config);

    let inputs = [
        "Soul music emerged in the 1950s.",
        "Rock and roll revolutionized popular music.",
        "Soul and rock both influenced modern music.",
    ];

    let request = EmbeddingsRequestBuilder::new()
        .input(EmbeddingsInput::Multiple(
            inputs.iter().map(|&s| s.to_string()).collect(),
        ))
        .model(EmbeddingModel::Voyage3)
        .input_type(InputType::Document)
        .build()
        .expect("Failed to build embeddings request");

    let response = client
        .create_embeddings(request.input)
        .await
        .expect("Failed to create embeddings");

    assert_eq!(
        response.data.len(),
        inputs.len(),
        "Number of embeddings should match number of inputs"
    );
    response.data.iter().enumerate().for_each(|(i, embedding)| {
        assert_eq!(
            embedding.index, i,
            "Embedding index should match input index"
        );
        assert!(
            !embedding.embedding.is_empty(),
            "Embedding should not be empty"
        );
    });
    Ok(())
}

#[tokio::test]
async fn test_embedding_single_input() {
    let api_key = std::env::var("VOYAGE_API_KEY").unwrap_or_else(|_| "test_key".to_string());
    let config = VoyageConfig::new(api_key);
    let client = VoyageAiClient::new_with_config(config);

    let input = "Soul rock music combines elements of both genres.";

    let request = EmbeddingsRequestBuilder::new()
        .input(EmbeddingsInput::Single(input.to_string()))
        .model(EmbeddingModel::Voyage3)
        .input_type(InputType::Document)
        .build()
        .expect("Failed to build embeddings request");

    let response = client
        .create_embeddings(request.input)
        .await
        .expect("Failed to create embedding");

    assert_eq!(response.data.len(), 1, "Should have a single embedding");
    assert!(
        !response.data[0].embedding.is_empty(),
        "Embedding should not be empty"
    );
}
