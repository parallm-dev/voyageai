use env_logger::Builder;
use log::{info, LevelFilter};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use std::io::Write;
use voyageai::builder::embeddings::EmbeddingsRequestBuilder;
use voyageai::models::embeddings::{EmbeddingModel, EmbeddingsInput};
use voyageai::models::rerank::{RerankModel, RerankRequest};
use voyageai::{InputType, VoyageAiClient, VoyageConfig};

#[tokio::test]
async fn test_voyage_ai_client() {
    // Set up logging
    let mut builder = Builder::from_default_env();
    builder
        .format(|buf, record| writeln!(buf, "{} - {}", record.level(), record.args()))
        .filter(None, LevelFilter::Debug)
        .init();

    // Initialize the client with the API key
    let api_key = std::env::var("VOYAGE_API_KEY").expect("VOYAGE_API_KEY must be set");
    let config = VoyageConfig::new(api_key.clone());
    let client = VoyageAiClient::new(config);

    // Test embeddings
    let embeddings_request = EmbeddingsRequestBuilder::new()
        .input(EmbeddingsInput::Multiple(vec![
            "Sample text 1".to_string(),
            "Sample text 2".to_string(),
        ]))
        .model(EmbeddingModel::Voyage3)
        .input_type(InputType::Document)
        .build()
        .expect("Failed to build embeddings request");

    let embeddings_response = client
        .embeddings()
        .create_embedding(&embeddings_request)
        .await
        .expect("Failed to get embeddings");

    assert_eq!(embeddings_response.object, "list");
    assert_eq!(embeddings_response.data.len(), 2);
    assert_eq!(embeddings_response.model, "voyage-3");
    println!(
        "Embeddings tokens used: {}",
        embeddings_response.usage.total_tokens
    );

    for (i, embedding_data) in embeddings_response.data.iter().enumerate() {
        assert_eq!(embedding_data.object, "embedding");
        assert_eq!(embedding_data.index, i);
        assert!(!embedding_data.embedding.is_empty());
        println!("Embedding {} length: {}", i, embedding_data.embedding.len());
    }

    // Test rerank
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
        top_k: None,
    };

    info!("Rerank request: {:?}", rerank_request);

    // Create a new reqwest client for logging purposes
    let reqwest_client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap(),
    );
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let response = reqwest_client
        .post("https://api.voyageai.com/v1/rerank")
        .headers(headers)
        .json(&rerank_request)
        .send()
        .await
        .expect("Failed to send request");

    info!("Response status: {}", response.status());
    info!("Response headers: {:#?}", response.headers());

    let response_body = response.text().await.expect("Failed to get response body");
    info!("Response body: {}", response_body);

    // Now proceed with the actual rerank request using the VoyageAiClient
    match client.rerank().rerank(&rerank_request).await {
        Ok(rerank_response) => {
            info!("Rerank response received successfully");
            debug!("Raw rerank response: {:?}", rerank_response);

            if rerank_response.data.is_empty() {
                panic!("Rerank results are empty");
            } else {
                info!("Rerank results received successfully");

                // Verify that the top result has the highest relevance score
                let top_result = &rerank_response.data[0];
                info!("Top result index: {}", top_result.index);
                assert!(
                    top_result.relevance_score >= rerank_response.data[1].relevance_score,
                    "Top result should have the highest relevance score"
                );

                // Print all rerank results for debugging
                info!("All rerank results:");
                for result in rerank_response.data.iter() {
                    info!("Result {}: Score: {}", result.index, result.relevance_score);
                }
            }

            info!("Rerank model used: {}", rerank_response.model);
            info!("Rerank tokens used: {}", rerank_response.usage.total_tokens);
        }
        Err(VoyageError::JsonError(err)) => {
            panic!("JSON error in rerank response: {:?}", err);
        }
        Err(err) => {
            panic!("Error in rerank request: {:?}", err);
        }
    }
}
