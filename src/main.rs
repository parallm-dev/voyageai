use clap::{Arg, Command};
use voyageai::models::rerank::{RerankModel, RerankRequest};
use voyageai::{VoyageAiClient, VoyageConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("voyageai")
        .version("1.0")
        .author("David Maple <david@parallm.dev>")
        .about("VoyageAI CLI for embeddings and reranking")
        .subcommand(
            Command::new("embed").about("Generate embeddings").arg(
                Arg::new("text")
                    .help("Text to embed")
                    .required(true)
                    .index(1),
            ),
        )
        .subcommand(
            Command::new("rerank")
                .about("Rerank documents")
                .arg(
                    Arg::new("query")
                        .help("Query for reranking")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::new("documents")
                        .help("Documents to rerank")
                        .required(true)
                        .multiple_values(true)
                        .index(2),
                ),
        )
        .get_matches();

    let config = VoyageConfig::new("your_api_key".to_string());
    let client = VoyageAiClient::new(config);

    if let Some(matches) = matches.subcommand_matches("embed") {
        let text = matches.value_of("text").unwrap();
        println!("Generating embedding for: {}", text);
        // TODO: Implement embedding functionality
    } else if let Some(matches) = matches.subcommand_matches("rerank") {
        let query = matches.value_of("query").unwrap();
        let documents: Vec<&str> = matches.values_of("documents").unwrap().collect();

        let request = RerankRequest {
            query: query.to_string(),
            documents: documents.iter().map(|&s| s.to_string()).collect(),
            model: RerankModel::Rerank2,
            top_k: None,
        };

        match client.rerank().rerank(&request).await {
            Ok(rerank_response) => {
                for result in rerank_response.data {
                    println!("Index: {}, Score: {}", result.index, result.relevance_score);
                }
                println!("Total tokens used: {}", rerank_response.usage.total_tokens);
            }
            Err(e) => {
                eprintln!("Error in rerank request: {:?}", e);
            }
        }
    }

    Ok(())
}
