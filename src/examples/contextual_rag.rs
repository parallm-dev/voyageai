use voyage::{embeddings, rerank, EmbeddingModel, RerankModel};

#[tokio::main]
async fn main() {
    // Build clients
    let embeddings_client = embeddings()
        .api_key("your_api_key_here")
        .build()
        .expect("Failed to build embeddings client");

    let rerank_client = rerank()
        .api_key("your_api_key_here")
        .build()
        .expect("Failed to build rerank client");

    // Example usage in a Retrieval-Augmented Generation context
    let query = "Explain the benefits of using Rust.";
    let documents = vec![
        "Rust offers memory safety without garbage collection.",
        "Python is a high-level programming language.",
        "Rust's ownership model prevents data races.",
    ];

    let embeddings_request = embeddings_client
        .input(&query)
        .model(EmbeddingModel::Voyage3)
        .build()
        .expect("Failed to build embeddings request");

    let embedding_response = embeddings_request.send().await.expect("Failed to get embeddings");

    let rerank_request = rerank_client
        .query(&query)
        .documents(documents.clone())
        .model(RerankModel::VoyageRerankV2)
        .build()
        .expect("Failed to build rerank request");

    let rerank_response = rerank_request.send().await.expect("Failed to rerank documents");

    // Use the top-ranked document for further processing
    let top_document = &documents[rerank_response.results[0].index];
    println!("Top document: {}", top_document);

    // ... continue with generating an answer using the top document ...
}
