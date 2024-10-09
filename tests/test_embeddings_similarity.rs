use approx::assert_relative_eq;
use voyageai::{EmbeddingModel, VoyageAiClient};

fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let magnitude_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let magnitude_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    dot_product / (magnitude_a * magnitude_b)
}

#[tokio::test]
async fn test_embeddings_similarity() {
    let client = VoyageAiClient::builder()
        .api_key("test_api_key")
        .build()
        .expect("Failed to build client");

    let obscure_document = "The tardigrade, also known as the water bear, is a microscopic animal that can survive extreme conditions, including the vacuum of space.";
    let similar_query = "Tiny creatures that can live in space";
    let dissimilar_query = "The economic impact of climate change on agriculture";

    // Generate embeddings for the document and queries
    let generate_embedding = |text: String| {
        let client = client.clone();
        async move {
            let request = client
                .embeddings()
                .input(text)
                .model(EmbeddingModel::Voyage3)
                .build()
                .expect("Failed to build embeddings request");

            client
                .embeddings()
                .create_embedding(&request)
                .await
                .expect("Failed to create embedding")
                .data[0]
                .embedding
                .clone()
        }
    };

    let document_embedding = generate_embedding(obscure_document.to_string()).await;
    let similar_query_embedding = generate_embedding(similar_query.to_string()).await;
    let dissimilar_query_embedding = generate_embedding(dissimilar_query.to_string()).await;

    // Calculate similarities
    let similarity_to_similar = cosine_similarity(&document_embedding, &similar_query_embedding);
    let similarity_to_dissimilar =
        cosine_similarity(&document_embedding, &dissimilar_query_embedding);

    // Assert that the similar query has a higher similarity score
    assert!(
        similarity_to_similar > similarity_to_dissimilar,
        "Expected similar query to have higher similarity. Similar: {}, Dissimilar: {}",
        similarity_to_similar,
        similarity_to_dissimilar
    );

    // Assert that the similar query has a relatively high similarity (adjust threshold as needed)
    assert_relative_eq!(similarity_to_similar, 1.0, epsilon = 0.3);

    // Assert that the dissimilar query has a relatively low similarity (adjust threshold as needed)
    assert!(
        similarity_to_dissimilar < 0.5,
        "Dissimilar query similarity too high: {}",
        similarity_to_dissimilar
    );
}
