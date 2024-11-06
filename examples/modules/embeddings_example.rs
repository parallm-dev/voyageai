use voyageai::{
    EmbeddingModel, EmbeddingsInput, EmbeddingsRequestBuilder,
    VoyageAiClient, VoyageConfig,
};

#[tokio::main]
pub async fn run_example() -> Result<(), Box<dyn std::error::Error>> {
    let config = VoyageConfig::new("your_api_key_here".to_string());
    let mut client = VoyageAiClient::new_with_config(config);

    let inputs = [
        "The quick brown fox jumps over the lazy dog.",
        "Voyage AI provides advanced AI services.",
        "Rust is a systems programming language.",
    ];

    let embeddings_request = EmbeddingsRequestBuilder::new()
        .input(EmbeddingsInput::Multiple(inputs.iter().map(|s| s.to_string()).collect()))
        .model(EmbeddingModel::Voyage3)
        .build()
        .expect("Failed to build embeddings request");

    let response = embeddings_request.send(&mut client).await?;
    for (i, embedding) in response.data.iter().enumerate() {
        println!("Embedding for '{}': {:?}", inputs[i], embedding.embedding);
    }
    Ok(())
}
