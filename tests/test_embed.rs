use voyageai::VoyageAiClient;

#[tokio::test]
async fn test_embeddings_with_multiple_inputs() {
    let client = VoyageAiClient::builder()
        .api_key("test_api_key")
        .build()
        .expect("Failed to build client");

    let inputs = vec![
        "first input".to_string(),
        "second input".to_string(),
        "third input".to_string(),
    ];
    let embeddings_request = voyageai::EmbeddingsRequestBuilder::new()
        .input_multiple(inputs.clone())
        .model(voyageai::EmbeddingModel::Voyage3)
        .with_client(client.clone())
        .build()
        .expect("Failed to build embeddings request");

    let response = client
        .embeddings()
        .create_embedding(&embeddings_request)
        .await;
    let embeddings_response = response.unwrap();
    assert_eq!(embeddings_response.data.len(), inputs.len());
}
