extern crate approx;

use approx::assert_relative_eq;
use serde_json::json;

use voyageai::builder::embeddings::EmbeddingsRequestBuilder;
use voyageai::models::embeddings::{EmbeddingModel, EmbeddingsInput};
use voyageai::{InputType, VoyageAiClient, VoyageConfig};

fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let magnitude_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let magnitude_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    dot_product / (magnitude_a * magnitude_b)
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito;

    pub fn setup_mock_server() -> mockito::ServerGuard {
        let mut server = mockito::Server::new();

        let mock_response = json!({
            "object": "list",
            "data": [
                {
                    "object": "embedding",
                    "embedding": [0.1, 0.2, 0.3],
                    "index": 0
                },
                {
                    "object": "embedding",
                    "embedding": [0.15, 0.25, 0.35],
                    "index": 1
                },
                {
                    "object": "embedding",
                    "embedding": [0.5, 0.6, 0.7],
                    "index": 2
                }
            ],
            "model": "voyage-3",
            "usage": {
                "prompt_tokens": 20,
                "total_tokens": 20
            }
        });

        server.mock("POST", "/v1/embeddings")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(mock_response.to_string())
            .create();

        server
    }

    #[tokio::test]
    async fn test_embeddings_similarity() {
    let mock_server = setup_mock_server();
    let config = VoyageConfig::new("mock_api_key".to_string())
        .with_base_url(mock_server.url());
    let client = VoyageAiClient::new(config);

    let texts = vec![
        "The quick brown fox jumps over the lazy dog",
        "A fast auburn canine leaps above an idle hound",
        "The sky is blue",
    ];

    // Mock response is already set up in the setup_mock_server function
    // No additional setup needed here

    let request = EmbeddingsRequestBuilder::new()
        .input(EmbeddingsInput::Multiple(
            texts.iter().map(|&s| s.to_string()).collect(),
        ))
        .model(EmbeddingModel::Voyage3)
        .input_type(InputType::Document)
        .build()
        .expect("Failed to build embeddings request");

    let response = client
        .embeddings()
        .create_embedding(&request)
        .await
        .expect("Failed to create embeddings");

    assert_eq!(response.data.len(), 3, "Expected 3 embeddings");

    let embedding1 = &response.data[0].embedding;
    let embedding2 = &response.data[1].embedding;
    let embedding3 = &response.data[2].embedding;

    let similarity_1_2 = cosine_similarity(embedding1, embedding2);
    let similarity_1_3 = cosine_similarity(embedding1, embedding3);
    let similarity_2_3 = cosine_similarity(embedding2, embedding3);

    println!("Similarity between 1 and 2: {}", similarity_1_2);
    println!("Similarity between 1 and 3: {}", similarity_1_3);
    println!("Similarity between 2 and 3: {}", similarity_2_3);

    assert!(
        similarity_1_2 > similarity_1_3,
        "Similarity between similar sentences should be higher"
    );
    assert!(
        similarity_1_2 > similarity_2_3,
        "Similarity between similar sentences should be higher"
    );

    // Assert that similarity between embedding1 and embedding2 is high (close to 1.0)
    assert_relative_eq!(similarity_1_2, 0.9999, epsilon = 0.0001);

    // Assert that similarity between embedding1 and embedding3 is lower
    assert_relative_eq!(similarity_1_3, 0.7303, epsilon = 0.0001);

    // Assert that similarity between embedding2 and embedding3 is lower
    assert_relative_eq!(similarity_2_3, 0.7321, epsilon = 0.0001);

    // Assert that similarity between similar embeddings is higher than dissimilar ones
    assert!(similarity_1_2 > similarity_1_3);
    assert!(similarity_1_2 > similarity_2_3);
}
