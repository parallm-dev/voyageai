use voyageai::{RerankModel, RerankRequestBuilder, VoyageAiClient};

#[tokio::main]
pub async fn run_example() -> Result<(), Box<dyn std::error::Error>> {
    let client = VoyageAiClient::new();

    let rerank_request = RerankRequestBuilder::new()
        .query("What is Rust?")
        .documents(vec![
            String::from("Rust is a systems programming language."),
            String::from("Rust is developed by Mozilla."),
            String::from("Dogs are mammals."),
        ])
        .model(RerankModel::Rerank2)
        .build()
        .expect("Failed to build rerank request");

    let response = client.rerank(rerank_request).await?;
    for result in &response.data {
        println!(
            "Document Index: {}, Relevance Score: {}",
            result.index, result.relevance_score
        );
    }
    for result in &response.data {
        println!(
            "Document Index: {}, Relevance Score: {}",
            result.index, result.relevance_score
        );
    }
    Ok(())
}
