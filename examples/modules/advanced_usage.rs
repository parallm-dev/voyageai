use voyageai::VoyageAiClient;

#[tokio::main]
pub async fn run_example() -> Result<(), Box<dyn std::error::Error>> {
    let client = VoyageAiClient::new();

    let texts = [
        "The quick brown fox jumps over the lazy dog",
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit",
        "To be or not to be, that is the question",
        "Call me Ishmael",
        "It was the best of times, it was the worst of times",
    ];

    println!("Processing chained operation...");

    let query = "What is the meaning of existence?";
    let documents = vec![
        "The meaning of life is 42".to_string(),
        "Life has no inherent meaning".to_string(),
        "The purpose of life is to be happy".to_string(),
    ];

    match client
        .chain()
        .embed_documents(texts.iter().map(|s| s.to_string()).collect::<Vec<_>>()).await
        .rerank_documents(query, documents).await
        .search("meaning of life")
        .await
        .execute()
        .await {
            Some(results) => {
                println!("Processed chain successfully");
                println!("Search query: {}", results.query.query);
                if let Some(docs) = results.documents {
                    for (i, doc) in docs.iter().enumerate() {
                        println!("Result {}: {}", i, doc);
                    }
                }
            }
            None => println!("Chain processing failed"),
        }
    Ok(())
}
