# ARCHITECTURE for `voyageai` lib



The package exports a module `voyage` that has builders for every operation you could possibly need. That is implemented in builder/voyage.rs

## Directory Structure

The `voyageai` library is organized into several key directories:

### src

The main source code directory containing the core functionality of the library.

#### builder

Contains builder patterns for constructing API requests:

- `embeddings.rs`: Builder for embedding requests
- `rerank.rs`: Builder for reranking requests
- `voyage.rs`: Main builder that aggregates all operations

#### client

Implements the HTTP clients for interacting with the Voyage AI API:

- `client_limiter.rs`: Rate limiting functionality with exponential backoff
- `embeddings_client.rs`: Client for embedding operations
- `rerank_client.rs`: Client for reranking operations
- `voyage_client.rs`: Main client that combines all API functionalities
- `retry.rs`: Implements retry logic with exponential backoff

#### config

Handles configuration and error management:

- `config.rs`: Configuration structures and methods
- `errors.rs`: Custom error types for the library

#### examples

Contains example scripts demonstrating library usage:

- `basic_client.rs`: Basic usage of the Voyage client
- `contextual_rag.rs`: Example of contextual retrieval-augmented generation
- `embeddings_example.rs`: Demonstration of embedding functionality
- `rerank_example.rs`: Example of reranking usage
- `rerank_with_scores.rs`: Reranking with score output
- `comprehensive_example.rs`: Demonstrates both embedding and reranking functionality

#### models

Defines data structures for API requests and responses:

- `embeddings.rs`: Structures for embedding operations
- `rerank.rs`: Structures for reranking operations

#### traits

Contains trait definitions:

- `llm.rs`: Defines the LLM (Language Model) trait

### tests

Contains unit and integration tests for the library:

- `test_builder.rs`: Tests for builder functionality
- `test_client.rs`: Tests for client operations
- `test_embed.rs`: Tests for embedding functionality
- `test_limiter.rs`: Tests for rate limiting
- `test_rerank.rs`: Tests for reranking functionality
- `integration_test.rs`: Integration tests for the entire library

This structure allows for a modular and maintainable codebase, separating concerns between API interactions, data models, and utility functions.

## Recent Improvements

We have made several improvements to align the library more closely with the VoyageAI API Guide:

1. Updated the `EmbeddingsRequest` struct and `EmbeddingsRequestBuilder` to include the `input_type` parameter.
2. Renamed `top_n` to `top_k` in the `RerankRequest` struct and `RerankRequestBuilder` for consistency with the API guide.
3. Implemented more comprehensive error handling that covers specific error codes mentioned in the API guide.
4. Implemented rate limit handling as per the API guide specifications:
   - Updated `client_limiter.rs` to handle both request-per-minute (RPM) and tokens-per-minute (TPM) limits for Embeddings and Reranking APIs.
   - Modified `EmbeddingClient` and `RerankClient` to use the rate limiter before making API calls.
   - Updated `VoyageAiClient` to incorporate the rate limiter for both embedding and reranking operations.
5. Implemented more accurate token estimation methods for both embedding and reranking requests.
6. Added retry logic with exponential backoff for rate limit errors and other transient failures.
7. Created a comprehensive example demonstrating both embedding and reranking functionality.
8. Updated tests to cover new parameters, error handling scenarios, and rate limiting functionality.
9. Improved documentation, including updates to README.md and inline code comments.

## Ongoing Improvements

While we have addressed many of the initial issues, there are still some areas that require attention:

1. Review and potentially remove or document the additional parameters (`truncate` and `include_metadata`) in the `RerankRequest` struct if they are not part of the official API.
2. Implement proper logging throughout the library to aid in debugging and monitoring.
3. Consider implementing a more sophisticated tokenization method or integrating a dedicated tokenizer library for even more accurate token estimation.

## Next Steps

To further improve the library's consistency with the VoyageAI API and enhance its robustness, we propose the following steps:

1. Conduct a thorough review of the API documentation to ensure all endpoints and parameters are correctly implemented.
2. Enhance error messages to provide more informative feedback to users.
3. Create more comprehensive examples demonstrating advanced usage scenarios and best practices.
4. Implement integration tests that mock the VoyageAI API to ensure correct behavior under various scenarios, including rate limiting and error conditions.
5. Consider adding support for streaming responses if the API supports it.
6. Explore possibilities for performance optimizations, especially for handling large volumes of requests.

By continuing to refine and improve the `voyageai` library, we aim to provide a robust, efficient, and user-friendly interface for interacting with the VoyageAI API.
# Architecture

The VoyageAI SDK is structured around several key components that work together to provide embeddings, reranking, and search functionality.

## Core Components

### Client Layer
- `VoyageAiClient`: The main entry point for interacting with the API
- `EmbeddingClient`: Handles embedding generation requests
- `RerankClient`: Manages document reranking operations
- `SearchClient`: Provides vector similarity search capabilities

### Builder Pattern
The SDK uses the Builder pattern extensively:
- `VoyageBuilder`: Constructs the main client
- `EmbeddingsRequestBuilder`: Creates embedding requests
- `RerankRequestBuilder`: Builds reranking requests
- `SearchRequestBuilder`: Constructs search queries

### Rate Limiting
- Built-in rate limiting with `RateLimiter`
- Handles both requests per minute (RPM) and tokens per minute (TPM)
- Automatic backoff and retry mechanisms

### Error Handling
- Custom error types with `VoyageError`
- Comprehensive error categorization
- Proper error propagation

## Data Flow

1. Client Initialization
   ```rust
   let client = VoyageBuilder::new()
       .with_api_key(api_key)
       .build()?;
   ```

2. Request Building
   ```rust
   let request = EmbeddingsRequestBuilder::new()
       .input(text)
       .model(EmbeddingModel::Voyage3)
       .build()?;
   ```

3. API Interaction
   ```rust
   let response = client.embed(request).await?;
   ```

## Design Principles

1. **Type Safety**: Extensive use of Rust's type system
2. **Ergonomic API**: Builder pattern for request construction
3. **Resource Management**: Automatic rate limiting
4. **Error Handling**: Rich error types and proper propagation
5. **Async First**: Built on Tokio for async/await support

## Testing Strategy

- Unit tests for individual components
- Integration tests for API interactions
- Property-based testing for complex operations
- Mocking for API responses
