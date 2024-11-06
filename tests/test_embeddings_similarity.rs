#[cfg(test)]
mod tests {
    use dotenvy::dotenv;
    use voyageai::{
        cosine_similarity, EmbeddingModel, EmbeddingsInput, EmbeddingsRequestBuilder, InputType,
        VoyageAiClient, VoyageConfig,
    };

    #[tokio::test]
    async fn test_embeddings_similarity() {
        dotenv().ok();
        let api_key = std::env::var("VOYAGE_API_KEY").expect("VOYAGE_API_KEY must be set");
        let config = VoyageConfig::new(api_key);
        let client = VoyageAiClient::new_with_config(config);

        let texts = [
            "The quick brown fox jumps over the lazy dog",
            "A fast auburn canine leaps above an idle hound",
            "The sky is blue",
        ];

        let request = EmbeddingsRequestBuilder::new()
            .input(EmbeddingsInput::Multiple(
                texts.iter().map(|&s| s.to_string()).collect(),
            ))
            .model(EmbeddingModel::Voyage3)
            .input_type(InputType::Document)
            .build()
            .expect("Failed to build embeddings request");

        let response = client
            .create_embeddings(request.input)
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

        // Test relative ordering of similarities only since actual values will vary
        assert!(
            similarity_1_2 > similarity_1_3,
            "Similarity between similar sentences should be higher"
        );
        assert!(
            similarity_1_2 > similarity_2_3,
            "Similarity between similar sentences should be higher"
        );
    }
}
