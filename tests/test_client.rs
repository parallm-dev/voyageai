#[cfg(test)]
mod tests {
    use std::error::Error;
    use voyageai::builder::embeddings::{EmbeddingsRequestBuilder, InputType};
    use voyageai::models::embeddings::EmbeddingModel;
    use voyageai::models::rerank::{RerankModel, RerankRequest};
    use voyageai::{VoyageAiClient, VoyageConfig};

    #[test]
    fn test_voyage_config() {
        let api_key = "test_api_key".to_string();
        let config = VoyageConfig::new(api_key.clone());
        assert_eq!(config.api_key(), &api_key, "API key should match");
    }

    #[tokio::test]
    async fn test_embeddings() -> Result<(), Box<dyn Error>> {
        let api_key = std::env::var("VOYAGE_API_KEY").expect("VOYAGE_API_KEY must be set");
        let config = VoyageConfig::new(api_key);
        let client = VoyageAiClient::new(config);

        let embeddings_request = EmbeddingsRequestBuilder::new()
            .input("test input")
            .model(EmbeddingModel::Voyage3)
            .input_type(InputType::Document)
            .build()?;

        let response = client
            .embeddings()
            .create_embedding(&embeddings_request)
            .await?;

        assert_eq!(
            response.data.len(),
            1,
            "Expected 1 embedding, got {}",
            response.data.len()
        );

        let embedding = &response.data[0];
        assert!(
            !embedding.embedding.is_empty(),
            "Embedding should not be empty"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_invalid_api_key() {
        let config = VoyageConfig::new("invalid_api_key".to_string());
        let client = VoyageAiClient::new(config);

        let embeddings_request = EmbeddingsRequestBuilder::new()
            .input("test input")
            .model(EmbeddingModel::Voyage3)
            .build()
            .expect("Failed to build embeddings request");

        let response = client
            .embeddings()
            .create_embedding(&embeddings_request)
            .await;

        assert!(
            matches!(response, Err(voyageai::VoyageError::Unauthorized)),
            "Expected Unauthorized error"
        );
    }

    #[tokio::test]
    async fn test_reranking() -> Result<(), Box<dyn Error>> {
        let api_key = std::env::var("VOYAGE_API_KEY").expect("VOYAGE_API_KEY must be set");
        let config = VoyageConfig::new(api_key);
        let client = VoyageAiClient::new(config);

        let rerank_request = RerankRequest {
            query: "test query".to_string(),
            documents: vec!["doc1".to_string(), "doc2".to_string()],
            model: RerankModel::Rerank2,
            top_k: Some(2),
        };

        let response = client.rerank().rerank(&rerank_request).await?;

        assert_eq!(
            response.results.len(),
            2,
            "Expected 2 reranked documents, got {}",
            response.results.len()
        );

        assert!(
            response.results[0].relevance_score >= response.results[1].relevance_score,
            "Documents should be sorted by relevance score"
        );

        Ok(())
    }

    // ... more tests ...
}
