use std::cmp::Ordering;
use std::collections::HashMap;

use crate::client::{embeddings_client::EmbeddingClient, rerank_client::RerankClient};
use crate::errors::VoyageError;

// Define SearchRequest struct here
#[derive(Debug, Clone)]
pub struct SearchRequest {
    pub query: String,
    pub documents: Vec<String>,
    pub top_k: Option<usize>,
    pub search_type: SearchType,
}

// Define SearchType enum here
#[derive(Debug, Clone)]
pub enum SearchType {
    Similarity,
    NearestNeighbor,
    NearestDuplicate,
    BM25,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchResult {
    pub document: String,
    pub score: i32, // Changed from f32 to i32 for Ord implementation
    pub index: usize,
}

impl PartialOrd for SearchResult {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SearchResult {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score)
    }
}

/// Client for performing search operations.
#[derive(Debug, Clone)]
pub struct SearchClient {
    #[allow(dead_code)]
    embedding_client: EmbeddingClient,
    #[allow(dead_code)]
    rerank_client: RerankClient,
    #[allow(dead_code)]
    document_index: HashMap<String, Vec<f32>>,
    #[allow(dead_code)]
    idf_scores: HashMap<String, f32>,
    #[allow(dead_code)]
    avg_doc_length: f32,
}

impl SearchClient {
    pub fn new(embedding_client: EmbeddingClient, rerank_client: RerankClient) -> Self {
        Self {
            embedding_client,
            rerank_client,
            document_index: HashMap::new(),
            idf_scores: HashMap::new(),
            avg_doc_length: 0.0,
        }
    }

    // ... (keep existing methods)

    /// Performs a nearest neighbor search based on Euclidean distance.
    #[allow(dead_code)]
    async fn nearest_neighbor_search(
        &self,
        _request: &SearchRequest,
    ) -> Result<Vec<SearchResult>, VoyageError> {
        // Implement nearest neighbor search logic here
        todo!("Implement nearest_neighbor_search")
    }

    /// Performs a nearest duplicate search to find similar documents.
    #[allow(dead_code)]
    async fn nearest_duplicate_search(
        &self,
        _request: &SearchRequest,
    ) -> Result<Vec<SearchResult>, VoyageError> {
        // Implement nearest duplicate search logic here
        todo!("Implement nearest_duplicate_search")
    }

    /// Performs a BM25 search for improved text relevance.
    #[allow(dead_code)]
    async fn bm25_search(
        &self,
        _request: &SearchRequest,
    ) -> Result<Vec<SearchResult>, VoyageError> {
        // Implement BM25 search logic here
        todo!("Implement bm25_search")
    }

    // ... (keep other existing methods)
}

/// Response structure for client-side search requests.
#[derive(Debug, Clone)]
pub struct SearchResponse {
    /// A list of search results.
    pub results: Vec<SearchResult>,
    /// Estimated token usage for the request.
    pub usage: u32,
}
