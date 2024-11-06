use env_logger::Builder;
use log::{debug, info, LevelFilter}; 
use voyageai::{
    client::SearchRequest,
    models::search::{SearchModel, SearchQuery, SearchType},
    traits::voyage::VoyageAiClientExt,
    EmbeddingModel, EmbeddingsInput, EmbeddingsRequestBuilder, InputType, RerankModel,
    RerankRequest, VoyageBuilder,
};
use std::io::Write;

#[tokio::test]
async fn test_voyage_ai_client() -> Result<(), Box<dyn std::error::Error>> {
    let client = VoyageBuilder::new()
        .with_api_key(std::env::var("VOYAGE_API_KEY").unwrap_or_else(|_| "test_key".to_string()))
        .build()
        .expect("Failed to build client");

    // Set up logging
    let mut builder = Builder::from_default_env();
    builder
        .format(|buf, record| writeln!(buf, "{} - {}", record.level(), record.args()))
        .filter(None, LevelFilter::Debug)
        .init();

    // Test embeddings
    let texts = vec![
        "Paris is the capital of France",
        "London is the capital of England",
        "Berlin is the capital of Germany",
    ];

    let embeddings_request = EmbeddingsRequestBuilder::default()
        .input(EmbeddingsInput::Multiple(texts.iter().map(ToString::to_string).collect()))
        .model(EmbeddingModel::Voyage3)
        .input_type(InputType::Document)
        .build()
        .expect("Failed to build embeddings request");

    info!("Submitting embeddings request");

    let embeddings_response = client
        .create_embeddings(embeddings_request.input)
        .await
        .expect("Failed to get embeddings");

    info!("Received embeddings response successfully");

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

    let rerank_request = RerankRequest::new(
        query.to_string(),
        documents.iter().map(|&s| s.to_string()).collect(),
        RerankModel::Rerank2,
        None,
    )
    .expect("Failed to create rerank request");

    info!("Rerank request: {:?}", rerank_request);

    // Now proceed with the rerank request using the VoyageAiClient
    let rerank_response = client.rerank(rerank_request).await?;
    {
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

            info!("Rerank model used: {}", rerank_response.model);
            info!("Rerank tokens used: {}", rerank_response.usage.total_tokens);
        }
    }

    // Test search
    let search_query = "capital of France".to_string();
    let search_request = SearchRequest {
        query: SearchQuery {
            query: search_query.clone(),
            model: SearchModel::default(),
            max_results: None,
            num_results: Some(10),
            include_metadata: None,
        },
        documents: Some(texts.iter().map(|&s| s.to_string()).collect()),
        embeddings: Some(embeddings_response.data.iter().map(|d| d.embedding.clone()).collect()),
        model: SearchModel::default(),
        top_k: None,
        search_type: SearchType::Similarity,
    };
    
    let search_response = client
        .search(search_request)
        .await
        .expect("Failed to perform search");

    info!("Search results:");
    for result in search_response {
        info!("Score: {}, Index: {}", result.score, result.index);
    }
    Ok(())
}
