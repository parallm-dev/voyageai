#[cfg(test)]

mod tests {
    use std::error::Error;
    use std::result::Result;
    use voyageai::builder::embeddings::EmbeddingsRequestBuilder;
    use voyageai::builder::voyage::VoyageAiClient;
    use voyageai::models::EmbeddingModel;

    #[tokio::test]
    async fn test_client() {
        // Test client implementation
    }

    #[tokio::test]
    async fn test_embeddings() -> Result<(), Box<dyn Error>> {
        let client = VoyageAiClient::builder()
            .api_key("test_api_key")
            .build()
            .expect("Failed to build client");

        let embeddings_request = EmbeddingsRequestBuilder::new()
            .input_multiple(vec!["test input".to_string()])
            .model(EmbeddingModel::Voyage3)
            .build()
            .expect("Failed to build embeddings request");

        let response = embeddings_request.send(&client).await?;
        let embeddings_response = response;
        assert_eq!(
            embeddings_response.data.len(),
            1,
            "Expected 1 embedding, got {}",
            embeddings_response.data.len()
        );

        let embedding = &embeddings_response.data[0];
        assert!(
            !embedding.embedding.is_empty(),
            "Embedding should not be empty"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_invalid_api_key() {
        let client = VoyageAiClient::builder()
            .api_key("invalid_api_key")
            .build()
            .expect("Failed to build client");

        let embeddings_request = client
            .embeddings()
            .input("test input")
            .model(EmbeddingModel::Voyage3)
            .build()
            .expect("Failed to build embeddings request");

        let response = client
            .embeddings()
            .create_embedding(&embeddings_request)
            .await;
        assert!(response.is_err());
    }
    // ... more tests ...
}
