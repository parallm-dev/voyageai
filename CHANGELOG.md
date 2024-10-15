# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2023-XX-XX

### Added

- Support for new embedding models: Voyage3, Voyage3Lite, VoyageFinance2, VoyageMultilingual2, VoyageLaw2
- Support for new rerank models: Rerank2, Rerank2Lite
- Improved error handling with more specific error types
- Rate limiting functionality to comply with API usage limits
- New examples demonstrating advanced usage and error handling

### Changed

- Updated EmbeddingClient and RerankClient to use new models and error handling
- Improved test coverage

### Fixed

- Various bug fixes and performance improvements

## [0.1.0] - 2023-XX-XX

### Added

- Initial release of the VoyageAI Rust SDK
- Basic support for embeddings and reranking
- Simple examples and tests
