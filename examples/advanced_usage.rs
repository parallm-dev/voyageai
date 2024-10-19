use std::env;
use std::time::Duration;
use voyageai::builder::embeddings::EmbeddingsRequestBuilder;
use voyageai::models::embeddings::EmbeddingModel;
use voyageai::models::rerank::{RerankModel, RerankRequest};
use voyageai::{VoyageAiClient, VoyageConfig, VoyageError};

async fn retry_with_exponential_backoff<F, Fut, T>(
    operation: F,
    max_retries: u32,
    initial_delay: Duration,
) -> Result<T, VoyageError>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = Result<T, VoyageError>>,
{
    let mut retries = 0;
    let mut delay = initial_delay;

    loop {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(VoyageError::RateLimitExceeded { reset_in }) => {
                if retries >= max_retries {
                    return Err(VoyageError::RateLimitExceeded { reset_in });
                }
                println!("Rate limit exceeded. Retrying in {:?}...", delay);
                tokio::time::sleep(delay).await;
                retries += 1;
                delay *= 2;
            }
            Err(e) => return Err(e),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up the client
    let api_key = env::var("VOYAGE_API_KEY").expect("VOYAGE_API_KEY must be set");
    let config = VoyageConfig::new(api_key);
    let client = VoyageAiClient::new(config);

    // Example text for embedding
    let texts = [
        "The quick brown fox jumps over the lazy dog",
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit",
        "To be or not to be, that is the question",
    ];

    println!("Creating embeddings for {} texts...", texts.len());

    // Create embeddings with retry logic and different models
    for (i, text) in texts.iter().enumerate() {
        let model = if i % 2 == 0 {
            EmbeddingModel::Voyage3
        } else {
            EmbeddingModel::Voyage3Lite
        };

        let result = retry_with_exponential_backoff(
            || async {
                let request = EmbeddingsRequestBuilder::new()
                    .input(text.to_string())
                    .model(model)
                    .build()
                    .map_err(|e| VoyageError::BuilderError(e.to_string()))?;

                client.embeddings().create_embedding(&request).await
            },
            3,
            Duration::from_secs(1),
        )
        .await;

        match result {
            Ok(response) => {
                println!(
                    "Embedding created for '{}' using model {:?}. Tokens used: {}",
                    text, model, response.usage.total_tokens
                );
            }
            Err(e) => {
                eprintln!("Error creating embedding for '{}': {:?}", text, e);
            }
        }
    }

    // Example for reranking with error handling
    let query = "What is the meaning of life?";
    let documents = [
        "The meaning of life is 42",
        "Life has no inherent meaning",
        "The purpose of life is to be happy",
        "The meaning of life is to find your gift. The purpose of life is to give it away",
    ];

    println!("\nReranking documents...");

    let rerank_result = retry_with_exponential_backoff(
        || async {
            let request = RerankRequest {
                query: query.to_string(),
                documents: documents.iter().map(|&s| s.to_string()).collect(),
                model: RerankModel::Rerank2,
                top_k: Some(2),
            };

            client.rerank().rerank(&request).await
        },
        3,
        Duration::from_secs(1),
    )
    .await;

    match rerank_result {
        Ok(response) => {
            println!("Documents reranked. Top results:");
            let data = response.data;

            // Now you can process `data` as needed
            println!("Response data: {:?}", data);

            println!("Tokens used: {}", response.usage.total_tokens);
        }
        Err(e) => {
            eprintln!("Error reranking documents: {:?}", e);
        }
    }

    // Demonstrate rate limiting
    println!("\nDemonstrating rate limiting...");

    for i in 1..=10 {
        let start = std::time::Instant::now();
        let request = match EmbeddingsRequestBuilder::new()
            .input(format!("Test input {}", i))
            .model(EmbeddingModel::Voyage3)
            .build()
        {
            Ok(req) => req,
            Err(e) => {
                eprintln!("Error building request: {:?}", e);
                continue;
            }
        };

        let result = client.embeddings().create_embedding(&request).await;
        let duration = start.elapsed();

        match result {
            Ok(_) => println!("Request {} completed in {:?}", i, duration),
            Err(VoyageError::RateLimitExceeded { reset_in }) => {
                println!(
                    "Rate limit reached on request {}. Reset in {:?}",
                    i, reset_in
                );
                tokio::time::sleep(reset_in).await;
            }
            Err(e) => eprintln!("Error on request {}: {:?}", i, e),
        }
    }

    for i in 1..=10 {
        let start = std::time::Instant::now();
        let request = match EmbeddingsRequestBuilder::new()
            .input(format!("Test input {}", i))
            .model(EmbeddingModel::Voyage3)
            .build()
        {
            Ok(req) => req,
            Err(e) => {
                eprintln!("Error building request: {:?}", e);
                continue;
            }
        };

        let result = client.embeddings().create_embedding(&request).await;
        let duration = start.elapsed();

        match result {
            Ok(_) => println!("Request {} completed in {:?}", i, duration),
            Err(VoyageError::RateLimitExceeded { reset_in }) => {
                println!(
                    "Rate limit reached on request {}. Reset in {:?}",
                    i, reset_in
                );
                tokio::time::sleep(reset_in).await;
            }
            Err(e) => eprintln!("Error on request {}: {:?}", i, e),
        }
    }

    for i in 1..=10 {
        let start = std::time::Instant::now();
        let request = match EmbeddingsRequestBuilder::new()
            .input(format!("Test input {}", i))
            .model(EmbeddingModel::Voyage3)
            .build()
        {
            Ok(req) => req,
            Err(e) => {
                eprintln!("Error building request: {:?}", e);
                continue;
            }
        };

        let result = client.embeddings().create_embedding(&request).await;
        let duration = start.elapsed();

        match result {
            Ok(_) => println!("Request {} completed in {:?}", i, duration),
            Err(VoyageError::RateLimitExceeded { reset_in }) => {
                println!(
                    "Rate limit reached on request {}. Reset in {:?}",
                    i, reset_in
                );
                tokio::time::sleep(reset_in).await;
            }
            Err(e) => eprintln!("Error on request {}: {:?}", i, e),
        }
    }

    for i in 1..=10 {
        let start = std::time::Instant::now();
        let request = match EmbeddingsRequestBuilder::new()
            .input(format!("Test input {}", i))
            .model(EmbeddingModel::Voyage3)
            .build()
        {
            Ok(req) => req,
            Err(e) => {
                eprintln!("Error building request: {:?}", e);
                continue;
            }
        };

        let result = client.embeddings().create_embedding(&request).await;
        let duration = start.elapsed();

        match result {
            Ok(_) => println!("Request {} completed in {:?}", i, duration),
            Err(VoyageError::RateLimitExceeded { reset_in }) => {
                println!(
                    "Rate limit reached on request {}. Reset in {:?}",
                    i, reset_in
                );
                tokio::time::sleep(reset_in).await;
            }
            Err(e) => eprintln!("Error on request {}: {:?}", i, e),
        }
    }

    Ok(())
}
