use voyageai::{RerankModel, RerankRequestBuilder, VoyageAiClient};

#[tokio::test]
async fn test_rerank() {
    let client = VoyageAiClient::builder()
        .api_key("test_api_key")
        .build()
        .expect("Failed to build rerank client");

    let rerank_request = RerankRequestBuilder::new()
        .query("test query")
        .documents(["doc1", "doc2", "doc3"])
        .model(RerankModel::V2)
        .build()
        .expect("Failed to build rerank request");

    let response = client.rerank().rerank(rerank_request).await;
    assert!(response.is_ok());
    let rerank_response = response.unwrap();
    assert_eq!(rerank_response.results.len(), 3);
}
