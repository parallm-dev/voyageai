mod modules {
    pub mod advanced_usage;
    pub mod basic_client;
    pub mod contextual_rag;
    pub mod embeddings_example;
    pub mod rerank_example;
    pub mod rerank_with_scores;
}
use modules::{
    advanced_usage, basic_client, contextual_rag, embeddings_example, rerank_example,
    rerank_with_scores,
};

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    match args.get(1).map(String::as_str) {
        Some("basic") => match basic_client::run_example().await {
            Ok(_) => {
                println!("Basic example completed successfully");
                Ok(())
            }
            Err(e) => {
                eprintln!("Failed to run basic example: {}", e);
                Err(e.into())
            }
        },
        Some("rag") => {
            contextual_rag::run_example()?;
            println!("RAG example completed successfully");
            Ok(())
        }
        Some("embeddings") => {
            embeddings_example::run_example()?;
            println!("Embeddings example completed successfully");
            Ok(())
        }
        Some("rerank") => {
            rerank_example::run_example()?;
            println!("Rerank example completed successfully");
            Ok(())
        }
        Some("scores") => {
            rerank_with_scores::run_example()?;
            println!("Scores example completed successfully");
            Ok(())
        }
        Some("advanced") => {
            advanced_usage::run_example()?;
            println!("Advanced example completed successfully");
            Ok(())
        }
        _ => {
            println!("Available commands:");
            println!("  basic      - Run basic client example");
            println!("  rag        - Run contextual RAG example");
            println!("  embeddings - Run embeddings example");
            println!("  rerank     - Run rerank example");
            println!("  scores     - Run rerank with scores example");
            println!("  advanced   - Run advanced usage example");
            Ok(())
        }
    }
}
