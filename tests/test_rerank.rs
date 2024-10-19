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

    match client.rerank().rerank(&rerank_request).await {
        Ok(response) => {
            assert!(!response.data.is_empty(), "Rerank operation returned empty results");
            assert_eq!(
                response.data.len(),
                2,
                "Expected 2 results due to top_k parameter"
            );
            assert!(
                response.data[0].relevance_score >= response.data[1].relevance_score,
                "Results should be sorted by relevance score"
            );
            assert_eq!(response.data[0].index, 0, "First result should be the most relevant document");
            assert!(
                response.usage.total_tokens > 0,
                "Usage information should be present"
            );
        },
        Err(e) => panic!("Rerank operation failed: {:?}", e),
    }

    match client.rerank().rerank(&rerank_request).await {
        Ok(response) => {
            assert!(!response.data.is_empty(), "Rerank operation returned empty results");
            assert_eq!(
                response.data.len(),
                2,
                "Expected 2 results due to top_k parameter"
            );
            assert!(
                response.data[0].relevance_score >= response.data[1].relevance_score,
                "Results should be sorted by relevance score"
            );
            assert_eq!(response.data[0].index, 0, "First result should be the most relevant document");
            assert!(
                response.usage.total_tokens > 0,
                "Usage information should be present"
            );
        },
        Err(e) => panic!("Rerank operation failed: {:?}", e),
    }

    match client.rerank().rerank(&rerank_request).await {
        Ok(response) => {
            assert!(!response.data.is_empty(), "Rerank operation returned empty results");
            assert_eq!(
                response.data.len(),
                2,
                "Expected 2 results due to top_k parameter"
            );
            assert!(
                response.data[0].relevance_score >= response.data[1].relevance_score,
                "Results should be sorted by relevance score"
            );
            assert_eq!(response.data[0].index, 0, "First result should be the most relevant document");
            assert!(
                response.usage.total_tokens > 0,
                "Usage information should be present"
            );
        },
        Err(e) => panic!("Rerank operation failed: {:?}", e),
    }

    match client.rerank().rerank(&rerank_request).await {
        Ok(response) => {
            assert!(!response.data.is_empty(), "Rerank operation returned empty results");
            assert_eq!(
                response.data.len(),
                2,
                "Expected 2 results due to top_k parameter"
            );
            assert!(
                response.data[0].relevance_score >= response.data[1].relevance_score,
                "Results should be sorted by relevance score"
            );
            assert_eq!(response.data[0].index, 0, "First result should be the most relevant document");
            assert!(
                response.usage.total_tokens > 0,
                "Usage information should be present"
            );
        },
        Err(e) => panic!("Rerank operation failed: {:?}", e),
    }

    Ok(())
}

#[tokio::test]
async fn test_rerank_invalid_input() -> Result<(), Box<dyn Error>> {
    let api_key = std::env::var("VOYAGE_API_KEY").expect("VOYAGE_API_KEY must be set");
    let config = VoyageConfig::new(api_key);
    let client = VoyageAiClient::new(config);

    let rerank_request = RerankRequest {
        query: "".to_string(),
        documents: vec![],
        model: RerankModel::Rerank2,
        top_k: Some(2),
    };

    let result = client.rerank().rerank(&rerank_request).await;
    assert!(result.is_err(), "Expected an error for invalid input");

    Ok(())
}

#[tokio::test]
async fn test_rerank_invalid_input() -> Result<(), Box<dyn Error>> {
    let api_key = std::env::var("VOYAGE_API_KEY").expect("VOYAGE_API_KEY must be set");
    let config = VoyageConfig::new(api_key);
    let client = VoyageAiClient::new(config);

    let rerank_request = RerankRequest {
        query: "".to_string(),
        documents: vec![],
        model: RerankModel::Rerank2,
        top_k: Some(2),
    };

    let result = client.rerank().rerank(&rerank_request).await;
    assert!(result.is_err(), "Expected an error for invalid input");

    Ok(())
}
