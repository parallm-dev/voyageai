use voyageai::{VoyageAiClient, RerankModel};

#[tokio::test]
async fn test_rerank() {
    let client = VoyageAiClient::builder()
        .api_key("test_api_key")
        .build()
        .expect("Failed to build rerank client");

    let rerank_request = client
        .rerank()
        .build()
        .expect("Failed to build rerank request")
        .query("test query")
        .documents(vec!["doc1".to_string(), "doc2".to_string(), "doc3".to_string()])
        .model(RerankModel::V2)
        .build()
        .expect("Failed to build rerank request");

    let response = rerank_request.send(&client).await;
    assert!(response.is_ok());
    let rerank_response = response.unwrap();
    assert_eq!(rerank_response.results.len(), 3);
}

// ... existing tests ...
