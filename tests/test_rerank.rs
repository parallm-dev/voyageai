use voyageai::{RerankModel, RerankRequest, VoyageAiClient};

#[tokio::test]
async fn test_rerank() -> Result<(), Box<dyn std::error::Error>> {
    let client = VoyageAiClient::new();

    let query = "What is the capital of France?";
    let documents = [
        "Paris is the capital of France.",
        "London is the capital of the United Kingdom.",
        "Berlin is the capital of Germany.",
    ];

    let rerank_request = RerankRequest::new(
        query.to_string(),
        documents.iter().map(|&s| s.to_string()).collect(),
        RerankModel::Rerank2,
        Some(2),
    )?;

    let response = client.rerank(rerank_request).await?;

    assert!(
        !response.data.is_empty(),
        "Rerank operation returned empty results"
    );
    assert_eq!(
        response.data.len(),
        2,
        "Expected 2 results due to top_k parameter"
    );
    assert!(
        response.data[0].relevance_score >= response.data[1].relevance_score,
        "Results should be sorted by relevance score"
    );
    assert!(
        response.data[0].index == 0,
        "Most relevant document should be the first one (about Paris)"
    );
    assert!(
        response.usage.total_tokens > 0,
        "Usage information should be present"
    );
    Ok(())
}

#[tokio::test]
async fn test_rerank_invalid_input() -> Result<(), Box<dyn std::error::Error>> {
    let _api_key = std::env::var("VOYAGE_API_KEY").expect("VOYAGE_API_KEY must be set");
    let _client = VoyageAiClient::new();

    let result = RerankRequest::new("".to_string(), vec![], RerankModel::Rerank2, Some(2));

    assert!(result.is_err());

    Ok(())
}
