use voyageai::{
    EmbeddingModel, EmbeddingsInput, EmbeddingsRequestBuilder,
    RerankModel, RerankRequest, VoyageAiClient, VoyageConfig,
};

#[tokio::main]
pub async fn run_example() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize client
    let api_key = std::env::var("VOYAGE_API_KEY").expect("VOYAGE_API_KEY must be set");
    let config = VoyageConfig::new(api_key);
    let client = VoyageAiClient::new_with_config(config);

    // Example usage in a Retrieval-Augmented Generation context
    let query = "Explain the benefits of using Rust.";
    let documents = [
        "Rust offers memory safety without garbage collection.",
        "Python is a high-level programming language.",
        "Rust's ownership model prevents data races.",
    ];

    // Create embeddings request
    let embeddings_request = EmbeddingsRequestBuilder::new()
        .input(EmbeddingsInput::Single(query.to_string()))
        .model(EmbeddingModel::Voyage3)
        .build()
        .expect("Failed to build embeddings request");

    let _embedding_response = client
        .create_embeddings(embeddings_request.input)
        .await
        .expect("Failed to get embeddings");

    // Create rerank request
    let rerank_request = RerankRequest::new(
        query.to_string(),
        documents.iter().map(|s| s.to_string()).collect(),
        RerankModel::Rerank2,
        None,
    )
    .expect("Failed to build rerank request");

    let rerank_response = client
        .rerank(rerank_request)
        .await
        .expect("Failed to rerank documents");

    // Use the top-ranked document for further processing
    let top_document = &documents[rerank_response.data[0].index];
    println!("Top document: {}", top_document);

    // Use the top document for response generation
    println!(
        "Generate LLM response augmented with data: {}",
        top_document
    );
    // Could integrate with an LLM API here for completion
    Ok(())
}
