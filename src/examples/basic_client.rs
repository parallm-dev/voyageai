use voyage::{embeddings, rerank, EmbeddingModel, RerankModel};

#[tokio::main]
async fn main() {
    // Build the embeddings client
    let embeddings_client = embeddings()
        .api_key("your_api_key_here")
        .build()
        .expect("Failed to build embeddings client");

    // Build the rerank client
    let rerank_client = rerank()
        .api_key("your_api_key_here")
        .build()
        .expect("Failed to build rerank client");

    // Embeddings request
    let embeddings_request = embeddings_client
        .input("This is a sample input for embeddings")
        .model(EmbeddingModel::Voyage3)
        .build()
        .expect("Failed to build embeddings request");

    // Rerank request
    let rerank_request = rerank_client
        .documents(vec!["Document 1", "Document 2", "Document 3"])
        .query("Query to rerank documents")
        .model(RerankModel::VoyageRerankV2)
        .build()
        .expect("Failed to build rerank request");

    // Send requests and handle responses
    let embeddings_result = embeddings_request.send().await;
    let rerank_result = rerank_request.send().await;

    match embeddings_result {
        Ok(response) => println!("Embeddings: {:?}", response.data),
        Err(err) => eprintln!("Embeddings error: {}", err),
    }

    match rerank_result {
        Ok(response) => println!("Reranked documents: {:?}", response.results),
        Err(err) => eprintln!("Rerank error: {}", err),
    }
}
