use voyageai::{VoyageAiClient, RerankRequestBuilder};
use voyageai::models::RerankModel;

#[tokio::main]
async fn main() {
    let client = VoyageAiClient::builder()
        .api_key("your_api_key_here")
        .build()
        .expect("Failed to build client");

    let rerank_request = client
        .rerank()
        .query("What is Rust?")
        .documents(vec![
            String::from("Rust is a systems programming language."),
            String::from("Rust is developed by Mozilla."),
            String::from("Dogs are mammals."),
        ])
        .model(RerankModel::VoyageRerankV2)
        .build()
        .expect("Failed to build rerank request");

    match rerank_request.send().await {
        Ok(response) => {
            for result in response.results {
                println!(
                    "Document Index: {}, Relevance Score: {}",
                    result.index, result.relevance_score
                );
            }
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
}