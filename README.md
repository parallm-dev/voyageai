
This SDK provides a Rust interface for the VoyageAI API, allowing you to easily integrate embedding and reranking capabilities into your Rust applications.

## Features

- Generate embeddings for text data using various models (e.g., Voyage3)
- Rerank documents based on relevance to a query using advanced models (e.g., RerankEnglishV2)
- Automatic rate limiting and error handling for efficient API usage
- Support for both synchronous and asynchronous operations
- Flexible request builders for easy API interaction
- Comprehensive examples and tests for reliable implementation
- Type-safe interfaces for embedding and reranking operations

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
voyageai = "0.1.0"
```

## Quick Start

```rust
use voyageai::{VoyageAiClient, VoyageConfig};
use voyageai::builder::embeddings::{EmbeddingsRequestBuilder, InputType};
use voyageai::models::embeddings::{EmbeddingModel, EmbeddingsInput};
use voyageai::builder::rerank::RerankRequestBuilder;
use voyageai::models::rerank::RerankModel;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("VOYAGE_API_KEY").expect("VOYAGE_API_KEY must be set");
    let config = VoyageConfig::new(api_key);
    let client = VoyageAiClient::new(config);

    let request = EmbeddingsRequestBuilder::new()
        .input(EmbeddingsInput::Single("Hello, world!".to_string()))
        .model(EmbeddingModel::Voyage3)
        .input_type(InputType::Document)
        .build()?;

    let response = client.embeddings().create_embedding(&request).await?;
    println!("Embedding: {:?}", response.data[0].embedding);

    // Example of using the reranking feature
    let rerank_request = RerankRequestBuilder::new()
        .query("What is the capital of France?")
        .documents(vec![
            "Paris is the capital of France.".to_string(),
            "London is the capital of England.".to_string(),
            "Berlin is the capital of Germany.".to_string(),
        ])
        .model(RerankModel::Rerank2)
        .build()?;

    let rerank_response = client.rerank().rerank(&rerank_request).await?;
    println!("Reranked results: {:?}", rerank_response.data);

    Ok(())
}
```

## Search Functionality

The VoyageAI Rust SDK now includes powerful search capabilities:

- Vector Similarity Search: Find documents similar to a given query or embedding.
- Nearest Neighbor Search: Locate the closest matches to a given point in the embedding space.
- Nearest Duplicate Detection: Identify and retrieve the most similar documents within a collection.
- BM25 Search: Perform text-based search using the BM25 ranking function for improved relevance.

Here's a quick example of how to use the search functionality:

```rust
use voyageai::{VoyageAiClient, VoyageConfig};
use voyageai::builder::search::{SearchRequestBuilder, SearchType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = VoyageAiClient::new(VoyageConfig::new("YOUR_API_KEY"));

    let search_request = SearchRequestBuilder::new()
        .query("artificial intelligence")
        .documents(vec!["example document".to_string()])
        .search_type(SearchType::Similarity)
        .top_k(5)
        .build()?;

    let search_results = client.search().search(&search_request).await?;
    println!("Search results: {:?}", search_results);

    Ok(())
}
```

This new search functionality allows for more advanced and flexible document retrieval, enabling you to build sophisticated search and recommendation systems with ease.

## Advanced Search Capabilities

The VoyageAI Rust SDK now offers simplified and powerful search capabilities through the `SearchClient`. This new functionality allows for easy implementation of various search types:

- Vector Similarity Search: Find documents similar to a given query or embedding.
- Nearest Neighbor Search: Locate the closest matches to a given point in the embedding space.
- Nearest Duplicate Detection: Identify and retrieve the most similar documents within a collection.
- BM25 Search: Perform text-based search using the BM25 ranking function for improved relevance.

Here's a quick example of how to use the new search functionality:

```rust
use voyageai::{VoyageAiClient, VoyageConfig};
use voyageai::client::search_client::SearchClient;
use voyageai::builder::search::SearchType;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = VoyageConfig::new("YOUR_API_KEY");
    let client = VoyageAiClient::new(config);
    let search_client = SearchClient::new(client.clone());

    // Vector Similarity Search
    let results = search_client.vector_similarity_search(
        "artificial intelligence",
        &[vec![0.1, 0.2, 0.3]],
        5
    ).await?;
    println!("Vector Similarity Search results: {:?}", results);

    // Nearest Neighbor Search
    let results = search_client.nearest_neighbor_search(
        &vec![0.1, 0.2, 0.3],
        &[vec![0.4, 0.5, 0.6], vec![0.7, 0.8, 0.9]],
        3
    ).await?;
    println!("Nearest Neighbor Search results: {:?}", results);

    // Nearest Duplicate Detection
    let results = search_client.nearest_duplicate_search(
        &[vec![0.1, 0.2, 0.3], vec![0.1, 0.2, 0.3]],
        0.95
    ).await?;
    println!("Nearest Duplicate Detection results: {:?}", results);

    // BM25 Search
    let results = search_client.bm25_search(
        "machine learning",
        &["Document 1 text", "Document 2 text", "Document 3 text"],
        5
    ).await?;
    println!("BM25 Search results: {:?}", results);

    Ok(())
}
```

This simplified search functionality allows for more intuitive and direct implementation of advanced search and recommendation systems, making it easier to integrate powerful search capabilities into your Rust applications.

## Client-Side Search Capabilities

The VoyageAI Rust SDK now provides powerful client-side search capabilities, leveraging the embedding and reranking features of the API. These search functions allow you to perform advanced document retrieval and similarity comparisons directly within your application:

1. Vector Similarity Search: Find documents that are semantically similar to a given query or embedding.
2. Nearest Neighbor Search: Identify the closest matches to a specific point in the embedding space.
3. Nearest Duplicate Detection: Discover and retrieve highly similar or duplicate documents within a collection.
4. BM25 Search: Execute text-based searches using the BM25 ranking function for improved relevance scoring.

Here's a quick example demonstrating how to use these new search capabilities:

```rust
use voyageai::{VoyageAiClient, VoyageConfig};
use voyageai::search::utils::{vector_similarity_search, nearest_neighbor_search, nearest_duplicate_search, bm25_search};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = VoyageConfig::new("YOUR_API_KEY");
    let client = VoyageAiClient::new(config);

    // Generate embeddings for your documents
    let documents = vec!["Document 1 content", "Document 2 content", "Document 3 content"];
    let embeddings = client.embeddings().embed_documents(&documents).await?;

    // Vector Similarity Search
    let query = "example query";
    let similar_docs = vector_similarity_search(&client, query, &embeddings, 5).await?;
    println!("Similar documents: {:?}", similar_docs);

    // Nearest Neighbor Search
    let query_embedding = client.embeddings().embed_text(query).await?;
    let neighbors = nearest_neighbor_search(&embeddings, &query_embedding, 3)?;
    println!("Nearest neighbors: {:?}", neighbors);

    // Nearest Duplicate Detection
    let duplicates = nearest_duplicate_search(&embeddings, 0.95)?;
    println!("Potential duplicates: {:?}", duplicates);

    // BM25 Search
    let bm25_results = bm25_search(query, &documents, 5)?;
    println!("BM25 search results: {:?}", bm25_results);

    Ok(())
}
```

These client-side search capabilities enable you to build sophisticated search and recommendation systems directly within your Rust applications, utilizing the power of VoyageAI's embedding and reranking models.

## Examples

Then you can run the examples using these cargo commands:

#### Run basic client example

```sh
cargo run --example basic_client
```

#### Run contextual RAG example

```sh
cargo run --example contextual_rag
```

#### Run embeddings example

```sh
cargo run --example embeddings_example
```

#### Run rerank example

```sh
cargo run --example rerank_example
```

#### Run rerank with scores example

```sh
cargo run --example rerank_with_scores
```

## Documentation

For more detailed information on how to use this SDK, please refer to the [documentation](https://docs.voyageai.rust). Please note that this link may not be active until the crate is published on crates.io. In the meantime, you can find documentation in the `docs` directory of the GitHub repository.

## Examples

Check out the `examples/` directory for more comprehensive examples of how to use this SDK.

## Testing

To run the tests, use:

```
cargo test
```

Note that some tests require a valid API key to be set in the `VOYAGE_API_KEY` environment variable.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Support

If you encounter any issues or have questions, please file an issue on the GitHub repository.
