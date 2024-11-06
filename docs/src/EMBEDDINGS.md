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
