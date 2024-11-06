use voyageai::{RerankModel, RerankRequest, VoyageAiClient};

#[tokio::main]
pub async fn run_example() -> Result<(), Box<dyn std::error::Error>> {
    let client = VoyageAiClient::with_key(
        std::env::var("VOYAGEAI_API_KEY").expect("Missing VOYAGEAI_API_KEY environment variable"),
    );

    let query = "benefits of using Rust for system programming";
    let documents = [
        "Rust offers memory safety without a garbage collector.",
        "Python is a popular scripting language.",
        "JavaScript is primarily used for web development.",
        "Rust's ownership model eliminates many classes of bugs.",
    ];

    let rerank_request = RerankRequest::new(
        query.to_string(),
        documents.iter().map(|&s| s.to_string()).collect(),
        RerankModel::Rerank2,
        Some(2),
    )
    .expect("Failed to build rerank request");

    let response = client.rerank(rerank_request).await?;
    println!("Reranked Documents (sorted by relevance):");
    let mut results = response.data;
    results.sort_by(|a, b| b.relevance_score.partial_cmp(&a.relevance_score).unwrap());
    for result in &results {
        println!(
            "Document: {}, Relevance Score: {}",
            documents[result.index], result.relevance_score
        );
    }
    for result in &results {
        println!(
            "Document: {}, Relevance Score: {}",
            documents[result.index], result.relevance_score
        );
    }
    Ok(())
}
