use voyageai::models::embeddings::EmbeddingModel;
use voyageai::models::rerank::{RerankModel, RerankRequest};
use voyageai::{builder::embeddings::EmbeddingsRequestBuilder, VoyageAiClient};

pub async fn run_example() -> Result<(Vec<f32>, Vec<f32>), Box<dyn std::error::Error>> {
    // Build the client
    let client = VoyageAiClient::new();

    // Embeddings request
    let embeddings_request = EmbeddingsRequestBuilder::new()
        .input("What are the main benefits of Rust programming language?")
        .model(EmbeddingModel::Voyage3)
        .build()?;

    // Rerank request with meaningful context
    let rerank_request = RerankRequest::new(
        "What are the key benefits of Rust programming language?".to_string(),
        vec![
            "Rust provides memory safety without garbage collection through its ownership system"
                .to_string(),
            "Rust enables writing high-performance system level code that is thread-safe"
                .to_string(),
            "Rust has zero-cost abstractions and no runtime overhead".to_string(),
        ],
        RerankModel::Rerank2,
        Some(2),
    )?;

    // Send requests and handle responses
    let embeddings_result = client.create_embeddings(embeddings_request.input).await?;

    let rerank_result = client.rerank(rerank_request).await?;

    // Extract and return the most relevant embeddings and scores
    let query_embedding = embeddings_result.data[0].embedding.clone();
    let top_doc_score = rerank_result.data[0].relevance_score as f32;

    Ok((query_embedding, vec![top_doc_score]))
}
