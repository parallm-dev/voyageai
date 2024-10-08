use voyageai::{VoyageAiClient, EmbeddingModel};

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_embeddings() {
        let client = VoyageAiClient::builder()
            .api_key("test_api_key")
            .build()
            .expect("Failed to build client");

        let embeddings_request = client
            .embeddings()
            .input("test input")
            .model(EmbeddingModel::Voyage3)
            .build()
            .expect("Failed to build embeddings request");

        let response = client.embeddings().create_embedding(&embeddings_request).await;
        assert!(response.is_ok());
        let embeddings_response = response.unwrap();
        assert_eq!(embeddings_response.data.len(), 1);
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

        let response = embeddings_request.send(&client).await;
        assert!(response.is_err());
    }

    // ... more tests ...
}
