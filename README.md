# API Client for Voyage AI


This Rust crate provides an unofficial client for interacting with the Voyage AI API. It allows you to easily integrate Voyage AI's powerful language models into your Rust applications.

## Features

- Simple and intuitive API
- Async support using Tokio
- Error handling with custom error types

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
voyageai = "0.1.0"
```

## Usage

First, set up your Voyage AI API credentials as environment variables:

```bash
export VOYAGE_API_KEY=your_api_key_here
```

Then, in your Rust code:

```rust
use voyage_ai_client::VoyageClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = VoyageClient::new()?;

    let response = client.generate_text("Once upon a time").await?;
    println!("Generated text: {}", response);

    Ok(())
}
```

## Configuration

You can customize the client by using the `VoyageClientBuilder`:

```rust
use voyage_ai_client::{VoyageClientBuilder, ApiVersion};

let client = VoyageClientBuilder::new()
    .api_version(ApiVersion::V1)
    .timeout(std::time::Duration::from_secs(30))
    .build()?;
```

## Error Handling

This crate uses custom error types to provide detailed error information. You can match on these errors to handle different error cases:

```rust
use voyage_ai_client::VoyageError;

match client.generate_text("prompt").await {
    Ok(response) => println!("Generated text: {}", response),
    Err(VoyageError::ApiError(e)) => eprintln!("API error: {}", e),
    Err(VoyageError::NetworkError(e)) => eprintln!("Network error: {}", e),
    Err(e) => eprintln!("Other error: {}", e),
}
```

## Documentation

For more detailed documentation, please run `cargo doc --open` or check the [online documentation](https://docs.rs/voyage_ai_client).

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
