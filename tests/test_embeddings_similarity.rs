use approx::assert_relative_eq;
use std::env;
use voyageai::builder::embeddings::{EmbeddingsRequestBuilder, InputType};
use voyageai::models::embeddings::EmbeddingModel;
use voyageai::{VoyageAiClient, VoyageConfig};

fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let magnitude_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let magnitude_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    dot_product / (magnitude_a * magnitude_b)
}

#[tokio::test]
async fn test_embeddings_similarity() {
    let api_key = env::var("VOYAGE_API_KEY").expect("VOYAGE_API_KEY must be set");
    let config = VoyageConfig::new(api_key);
    let client = VoyageAiClient::new(config);

    let texts = vec![
        "The quick brown fox jumps over the lazy dog",
        "A fast auburn canine leaps above an idle hound",
        "The sky is blue",
    ];

    let request = EmbeddingsRequestBuilder::new()
        .input_multiple(texts.clone())
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

    assert_relative_eq!(similarity_1_2, 1.0, epsilon = 0.1);
    assert!(
        similarity_1_3 < 0.8,
        "Unrelated sentences should have lower similarity"
    );
    assert!(
        similarity_2_3 < 0.8,
        "Unrelated sentences should have lower similarity"
    );
}
