use voyageai::builder::embeddings::EmbeddingsRequestBuilder;
use voyageai::models::embeddings::EmbeddingModel;
use voyageai::models::rerank::{RerankModel, RerankRequest};
use voyageai::{InputType, VoyageAiClient, VoyageConfig, VoyageError};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up the client using the Default implementation of VoyageConfig
    let config = VoyageConfig::default();
    if config.api_key().is_empty() {
        eprintln!("Warning: VOYAGE_API_KEY environment variable is not set or empty.");
        eprintln!("Please set the VOYAGE_API_KEY environment variable to use this example.");
        std::process::exit(1);
    }
    let client = VoyageAiClient::new(config);

    // Example text for embedding
    let texts = [
        "The quick brown fox jumps over the lazy dog",
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit",
        "To be or not to be, that is the question",
    ];

    println!("Creating embeddings for {} texts...", texts.len());

    // Create embeddings
    for text in &texts {
        let request = EmbeddingsRequestBuilder::new()
            .input(text.to_string())
            .model(EmbeddingModel::Voyage3)
            .input_type(InputType::Document)
            .build()?;

        match client.embeddings().create_embedding(&request).await {
            Ok(response) => {
                println!(
                    "Embedding created for '{}'. Tokens used: {}",
                    text, response.usage.total_tokens
                );
            }
            Err(VoyageError::RateLimitExceeded { reset_in }) => {
                println!(
                    "Rate limit exceeded. Waiting for {} seconds...",
                    reset_in.as_secs()
                );
                tokio::time::sleep(reset_in).await;
            }
            Err(e) => {
                eprintln!("Error creating embedding: {}", e);
            }
        }
    }

    // Example for reranking
    let query = "What is the meaning of life?";
    let documents = [
        "The meaning of life is 42",
        "Life has no inherent meaning",
        "The purpose of life is to be happy",
        "The meaning of life is to find your gift. The purpose of life is to give it away",
    ];

    println!("\nReranking documents...");

    let rerank_request = RerankRequest {
        query: query.to_string(),
        documents: documents.iter().map(|&s| s.to_string()).collect(),
        model: RerankModel::Rerank2,
        top_k: Some(2),
    };

    match client.rerank().rerank(&rerank_request).await {
        Ok(response) => {
            println!("Documents reranked. Top results:");
            for result in response.data.iter().take(2) {
                println!(
                    "- {} (score: {})",
                    documents[result.index], result.relevance_score
                );
            }
            println!("Tokens used: {}", response.usage.total_tokens);
        }
        Err(VoyageError::RateLimitExceeded { reset_in }) => {
            println!(
                "Rate limit exceeded for reranking. Waiting for {} seconds...",
                reset_in.as_secs()
            );
        }
        Err(e) => {
            eprintln!("Error reranking documents: {}", e);
        }
    }

    Ok(())
}
