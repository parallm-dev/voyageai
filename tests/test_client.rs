use std::error::Error;
use voyageai::{
    EmbeddingModel, EmbeddingsRequestBuilder, RerankModel, RerankRequest, VoyageAiClient,
    VoyageConfig, VoyageError,
};

#[tokio::test]
async fn test_embeddings() -> Result<(), Box<dyn Error>> {
    let config = VoyageConfig::new(
        std::env::var("VOYAGE_API_KEY").unwrap_or_else(|_| "test_key".to_string()),
    );
    let client = VoyageAiClient::new_with_config(config);

    let embeddings_request = EmbeddingsRequestBuilder::new()
        .document("test input")
        .model(EmbeddingModel::Voyage3)
        .build()?;

    let response = client.create_embeddings(embeddings_request.input).await?;

    assert_eq!(response.data.len(), 1, "Expected one embedding");
    assert!(
        !response.data[0].embedding.is_empty(),
        "Embedding should not be empty"
    );

    Ok(())
}

#[tokio::test]
async fn test_invalid_api_key() -> Result<(), Box<dyn Error>> {
    let config = VoyageConfig::new("invalid_api_key".to_string());
    let client = VoyageAiClient::new_with_config(config);

    let embeddings_request = EmbeddingsRequestBuilder::new()
        .document("test input")
        .model(EmbeddingModel::Voyage3)
        .build()?;

    let response = client.create_embeddings(embeddings_request.input).await;

    assert!(
        matches!(response, Err(err) if err.is::<VoyageError>() && matches!(err.downcast_ref::<VoyageError>().unwrap(), VoyageError::Unauthorized))
    );
    Ok(())
}

#[tokio::test]
async fn test_reranking() -> Result<(), Box<dyn Error>> {
    let config = VoyageConfig::new(
        std::env::var("VOYAGE_API_KEY").unwrap_or_else(|_| "test_key".to_string()),
    );
    let client = VoyageAiClient::new_with_config(config);

    let rerank_request = RerankRequest::new(
        "test query".to_string(),
        vec!["doc1".to_string(), "doc2".to_string()],
        RerankModel::Rerank2,
        Some(2),
    )?;

    let response = client.rerank(rerank_request).await?;
    let results = response.data;

    assert_eq!(results.len(), 2, "Expected exactly 2 reranked documents");
    assert!(
        results[0].relevance_score > results[1].relevance_score,
        "Documents should be sorted by relevance score"
    );

    Ok(())
}
