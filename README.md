# VoyageAI Rust SDK

This SDK provides a Rust interface for the VoyageAI API, allowing you to easily integrate embedding and reranking capabilities into your Rust applications.

## Features

- Generate embeddings for text data using various models
- Rerank documents based on relevance to a query
- Automatic rate limiting and error handling
- Comprehensive examples and tests

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
voyageai = "0.2.0"
```

## Quick Start

```rust
use voyageai::{VoyageAiClient, VoyageConfig};
use voyageai::builder::embeddings::{EmbeddingsRequestBuilder, InputType};
use voyageai::models::embeddings::EmbeddingModel;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("VOYAGE_API_KEY").expect("VOYAGE_API_KEY must be set");
    let config = VoyageConfig::new(api_key);
    let client = VoyageAiClient::new(config);

    let request = EmbeddingsRequestBuilder::new()
        .input("Hello, world!")
        .model(EmbeddingModel::Voyage3)
        .input_type(InputType::Document)
        .build()?;

    let response = client.embeddings().create_embedding(&request).await?;
    println!("Embedding: {:?}", response.data[0].embedding);

    Ok(())
}
```

## Documentation

For more detailed information on how to use this SDK, please refer to the [documentation](https://docs.rs/voyageai).

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
