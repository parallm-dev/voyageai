# ARCHITECTURE for `voyageai` lib

 voyageai
├──  ARCHITECTURE.md
├──  Cargo.toml
├──  LICENSE.md
├──  README.md
├── 󱧼 src
│   ├──  builder
│   │   ├──  embeddings.rs
│   │   ├──  mod.rs
│   │   ├──  rerank.rs
│   │   └──  voyage.rs
│   ├──  client
│   │   ├──  client_limiter.rs
│   │   ├──  embeddings_client.rs
│   │   ├──  mod.rs
│   │   ├──  rerank_client.rs
│   │   └──  voyage_client.rs
│   ├──  config
│   │   ├──  config.rs
│   │   ├──  errors.rs
│   │   └──  mod.rs
│   ├──  examples
│   │   ├──  basic_client.rs
│   │   ├──  contextual_rag.rs
│   │   ├──  embeddings_example.rs
│   │   ├──  rerank_example.rs
│   │   └──  rerank_with_scores.rs
│   ├──  lib.rs
│   ├──  models
│   │   ├──  embeddings.rs
│   │   ├──  mod.rs
│   │   └──  rerank.rs
│   └──  traits
│       └──  llm.rs
├── 󰙨 tests
│   ├──  test_builder.rs
│   ├──  test_client.rs
│   ├──  test_embed.rs
│   ├──  test_limiter.rs
│   └──  test_rerank.rs
└──  ZED_CONVENTIONS.md

the package exports a module `voyage` that has builders for every operation you could possibly need. That is implemented in builder/voyage.rs

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

- `client_limiter.rs`: Rate limiting functionality
- `embeddings_client.rs`: Client for embedding operations
- `rerank_client.rs`: Client for reranking operations
- `voyage_client.rs`: Main client that combines all API functionalities

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

This structure allows for a modular and maintainable codebase, separating concerns between API interactions, data models, and utility functions.
