use voyage::{embeddings, EmbeddingModel};

#[tokio::test]
async fn test_embeddings_with_multiple_inputs() {
    let client = embeddings()
        .api_key("test_api_key")
        .build()
        .expect("Failed to build embeddings client");

    let inputs = vec!["first input", "second input", "third input"];
    let embeddings_request = client
        .input(inputs.clone())
        .model(EmbeddingModel::Voyage3)
        .build()
        .expect("Failed to build embeddings request");

    let response = embeddings_request.send().await;
    assert!(response.is_ok());
    let embeddings_response = response.unwrap();
    assert_eq!(embeddings_response.data.len(), inputs.len());
}
