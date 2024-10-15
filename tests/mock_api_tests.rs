extern crate mockito;

use voyageai::{
    builder::embeddings::EmbeddingsRequestBuilder,
    models::{
        embeddings::EmbeddingModel,
        rerank::{RerankModel, RerankRequest},
    },
    VoyageAiClient, VoyageConfig,
};

#[tokio::test]
async fn test_embeddings_api() {
    let mut server = mockito::Server::new();
    let mock_url = server.url();

    let _m = server
        .mock("POST", "/embeddings")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"
            {
                "object": "list",
                "data": [
                    {
                        "object": "embedding",
                        "embedding": [0.1, 0.2, 0.3],
                        "index": 0
                    }
                ],
                "model": "voyage-3",
                "usage": {
                    "total_tokens": 5
                }
            }
            "#,
        )
        .create();

    let config = VoyageConfig::new("test_api_key".to_string()).with_base_url(mock_url);
    let client = VoyageAiClient::new(config);

    let request = EmbeddingsRequestBuilder::new()
        .input("Test input")
        .model(EmbeddingModel::Voyage3)
        .build()
        .unwrap();

    let response = client
        .embeddings()
        .create_embedding(&request)
        .await
        .unwrap();

    assert_eq!(response.object, "list");
    assert_eq!(response.data.len(), 1);
    assert_eq!(response.data[0].embedding, vec![0.1, 0.2, 0.3]);
    assert_eq!(response.model, "voyage-3");
    assert_eq!(response.usage.total_tokens, 5);
}

#[tokio::test]
async fn test_rerank_api() {
    let mut server = mockito::Server::new();
    let mock_url = server.url();

    let _m = server
        .mock("POST", "/rerank")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"
            {
                "results": [
                    {
                        "document": "Paris is the capital of France.",
                        "relevance_score": 0.95,
                        "index": 0
                    }
                ],
                "model": "rerank-2",
                "usage": {
                    "total_tokens": 10
                }
            }
            "#,
        )
        .create();

    let config = VoyageConfig::new("test_api_key".to_string()).with_base_url(mock_url);
    let client = VoyageAiClient::new(config);

    let request = RerankRequest {
        query: "What is the capital of France?".to_string(),
        documents: vec!["Paris is the capital of France.".to_string()],
        model: RerankModel::Rerank2,
        top_k: None,
    };

    let response = client.rerank().rerank(&request).await.unwrap();

    assert_eq!(response.results.len(), 1);
    assert_eq!(
        response.results[0].document,
        "Paris is the capital of France."
    );
    assert_eq!(response.results[0].relevance_score, 0.95);
    assert_eq!(response.results[0].index, 0);
    assert_eq!(response.model, "rerank-2");
    assert_eq!(response.usage.total_tokens, 10);
}
