# Voyage AI Embeddings Rust SDK

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
voyageai = { git = "https://github.com/parallm-dev/voyageai", branch = "main" }
```

## Quick Start

```rust
use voyageai::{VoyageAiClient, VoyageConfig};
use voyageai::builder::embeddings::{EmbeddingsRequestBuilder, InputType};
use voyageai::models::embeddings::{EmbeddingModel, EmbeddingsInput};
use voyageai::builder::rerank::RerankRequest
# Embeddings

## Overview

The VoyageAI SDK provides powerful text embedding capabilities through various models optimized for different use cases.

## Available Models

- `Voyage3`: High-quality general purpose embeddings
- `Voyage3Lite`: Faster, lighter version with good quality
- `VoyageFinance2`: Optimized for financial text
- `VoyageMultilingual2`: Support for multiple languages
- `VoyageLaw2`: Specialized for legal documents

## Basic Usage

```rust
let client = VoyageBuilder::new()
    .with_api_key(api_key)
    .build()?;

let request = EmbeddingsRequestBuilder::new()
    .input("Your text here")
    .model(EmbeddingModel::Voyage3)
    .build()?;

let response = client.embed(request).await?;
```

## Advanced Features

### Batch Processing
```rust
let texts = vec!["Text 1", "Text 2", "Text 3"];
let request = EmbeddingsRequestBuilder::new()
    .input(EmbeddingsInput::Multiple(texts))
    .model(EmbeddingModel::Voyage3)
    .build()?;
```

### Input Types
```rust
let request = EmbeddingsRequestBuilder::new()
    .input(text)
    .input_type(InputType::Document)
    .build()?;
```

### Encoding Formats
```rust
let request = EmbeddingsRequestBuilder::new()
    .input(text)
    .encoding_format(EncodingFormat::Float)
    .build()?;
```

## Model Characteristics

| Model | Dimensions | Context Length | TPM Limit |
|-------|------------|----------------|-----------|
| Voyage3 | 512 | 32000 | 320k |
| Voyage3Lite | 256 | 32000 | 1M |
| VoyageFinance2 | 1024 | 16000 | 120k |
| VoyageMultilingual2 | 1024 | 16000 | 120k |
| VoyageLaw2 | 1024 | 16000 | 120k |

## Best Practices

1. Choose the appropriate model for your use case
2. Use batch processing for multiple texts
3. Consider input type based on your content
4. Handle rate limits appropriately
5. Implement proper error handling
