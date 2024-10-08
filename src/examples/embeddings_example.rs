use voyageai::{VoyageAiClient, EmbeddingModel};

#[tokio::main]
async fn main() {
    let client = VoyageAiClient::builder()
        .api_key("your_api_key_here")
        .build()
        .expect("Failed to build client");

    let inputs = vec![
        "The quick brown fox jumps over the lazy dog.",
        "Voyage AI provides advanced AI services.",
        "Rust is a systems programming language.",
    ];

    let embeddings_request = client
        .embeddings()
        .input(inputs.clone())
        .model(EmbeddingModel::Voyage3)
        .build()
        .expect("Failed to build embeddings request");

    match embeddings_request.send().await {
        Ok(response) => {
            for (i, embedding) in response.data.iter().enumerate() {
                println!("Embedding for '{}': {:?}", inputs[i], embedding.embedding);
            }
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
}
