use voyageai::{VoyageAiClient, RerankRequestBuilder};
use voyageai::models::RerankModel;

#[tokio::main]
async fn main() {
    let client = VoyageAiClient::builder()
        .api_key("your_api_key_here")
        .build()
        .expect("Failed to build client");

    let query = "benefits of using Rust for system programming";
    let documents = vec![
        "Rust offers memory safety without a garbage collector.",
        "Python is a popular scripting language.",
        "JavaScript is primarily used for web development.",
        "Rust's ownership model eliminates many classes of bugs.",
    ];

    let rerank_request = client
        .rerank()
        .query(query)
        .documents(documents.clone())
        .model(RerankModel::VoyageRerankV2)
        .build()
        .expect("Failed to build rerank request");

    match rerank_request.send().await {
        Ok(response) => {
            println!("Reranked Documents (sorted by relevance):");
            let mut results = response.results;
            results.sort_by(|a, b| b.relevance_score.partial_cmp(&a.relevance_score).unwrap());
            for result in results {
                println!(
                    "Document: {}, Relevance Score: {}",
                    documents[result.index], result.relevance_score
                );
            }
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
}