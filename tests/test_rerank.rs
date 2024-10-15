use std::error::Error;
use voyageai::{
    models::rerank::{RerankModel, RerankRequest},
    VoyageAiClient, VoyageConfig,
};

#[tokio::test]
async fn test_rerank() -> Result<(), Box<dyn Error>> {
    let api_key = std::env::var("VOYAGE_API_KEY").expect("VOYAGE_API_KEY must be set");
    let config = VoyageConfig::new(api_key);
    let client = VoyageAiClient::new(config);

    let query = "What is the capital of France?";
    let documents = [
        "Paris is the capital of France.",
        "London is the capital of the United Kingdom.",
        "Berlin is the capital of Germany.",
    ];

    let rerank_request = RerankRequest {
        query: query.to_string(),
        documents: documents.iter().map(|&s| s.to_string()).collect(),
        model: RerankModel::Rerank2,
        top_k: Some(2),
    };

    let response = client.rerank().rerank(&rerank_request).await?;

    assert_eq!(
        response.results.len(),
        2,
        "Expected 2 results due to top_k parameter"
    );
    assert!(
        response.results[0].relevance_score >= response.results[1].relevance_score,
        "Results should be sorted by relevance score"
    );

    Ok(())
}
