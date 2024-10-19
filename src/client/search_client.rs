use std::collections::{HashMap, HashSet};

use crate::client::{embeddings_client::EmbeddingClient, rerank_client::RerankClient};
use crate::builder::search::SearchRequest;
use crate::models::search::{SearchResult, SearchType};
use crate::errors::VoyageError;

// Define SearchRequest struct here

/// Client for performing search operations.
#[derive(Debug, Clone)]
pub struct SearchClient {
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
    async fn nearest_neighbor_search(
        &self,
        request: &SearchRequest,
    ) -> Result<Vec<SearchResult>, VoyageError> {
        // Obtain embeddings for the query and documents
        let query_embedding: Vec<f32> = self.embedding_client.embed(&request.query).await?;
        let document_embeddings: Vec<Vec<f32>> = self
            .embedding_client
            .embed_batch(&request.documents)
            .await?;

        // Calculate distances
        let mut results = request
            .documents
            .iter()
            .zip(document_embeddings)
            .enumerate()
            .map(|(index, (doc, doc_embedding))| {
                let distance = Self::euclidean_distance(&query_embedding, &doc_embedding);
                SearchResult {
                    document: doc.to_string(),
                    score: distance as i32, // Convert to i32 for consistency
                    index,
                    search_type: SearchType::NearestNeighbor,
                }
            })
            .collect::<Vec<_>>();

        // Sort results by distance (ascending)
        results.sort_by(|a, b| a.score.cmp(&b.score));

        // Truncate to top_k if specified
        if let Some(top_k) = request.top_k {
            results.truncate(top_k);
        }

        Ok(results)
    }

    // Helper function to calculate Euclidean distance
    fn euclidean_distance(a: &[f32], b: &[f32]) -> f32 {
        a.iter()
            .zip(b)
            .map(|(x, y)| (x - y).powi(2))
            .sum::<f32>()
            .sqrt()
    }

    /// Performs a nearest duplicate search to find similar documents.
    #[allow(dead_code)]
    async fn nearest_duplicate_search(
        &self,
        request: &SearchRequest,
    ) -> Result<Vec<SearchResult>, VoyageError> {
        // Obtain embeddings for the query and documents
        let query_embedding = self.embedding_client.embed(&request.query).await?;
        let document_embeddings = self
            .embedding_client
            .embed_batch(&request.documents)
            .await?;

        // Calculate cosine similarities
        let mut results = request
            .documents
            .iter()
            .zip(document_embeddings)
            .enumerate()
            .map(|(index, (doc, doc_embedding))| {
                let similarity = Self::cosine_similarity(&query_embedding, &doc_embedding);
                SearchResult {
                    document: doc.to_string(),
                    score: similarity as i32, // Convert to i32 for consistency
                    index,
                    search_type: SearchType::NearestDuplicate,
                }
            })
            .collect::<Vec<_>>();

        // Sort results by similarity (descending)
        results.sort_by(|a, b| b.score.cmp(&a.score));

        // Truncate to top_k if specified
        if let Some(top_k) = request.top_k {
            results.truncate(top_k);
        }

        Ok(results)
    }

    // Helper function to calculate cosine similarity
    fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
        let dot_product = a.iter().zip(b).map(|(x, y)| x * y).sum::<f32>();
        let magnitude_a = a.iter().map(|x| x.powi(2)).sum::<f32>().sqrt();
        let magnitude_b = b.iter().map(|x| x.powi(2)).sum::<f32>().sqrt();
        dot_product / (magnitude_a * magnitude_b)
    }

    /// Performs a BM25 search for improved text relevance.
    #[allow(dead_code)]
    async fn bm25_search(
        &mut self,
        request: &SearchRequest,
    ) -> Result<Vec<SearchResult>, VoyageError> {
        // Ensure the IDF scores and average document length are calculated
        if self.idf_scores.is_empty() || self.avg_doc_length == 0.0 {
            self.compute_bm25_parameters(&request.documents);
        }

        // Tokenize the query
        let query_terms = Self::tokenize(&request.query);

        // Calculate BM25 scores
        let mut results = request
            .documents
            .iter()
            .enumerate()
            .map(|(index, doc)| {
                let score = self.compute_bm25_score(doc, &query_terms);
                SearchResult {
                    document: doc.clone(),
                    score: score as i32, // Convert to i32 for consistency
                    index,
                }
            })
            .collect::<Vec<_>>();

        // Sort results by score (descending)
        results.sort_by(|a, b| b.score.cmp(&a.score));

        // Truncate to top_k if specified
        if let Some(top_k) = request.top_k {
            results.truncate(top_k);
        }

        Ok(results)
    }

    // Helper methods for BM25

    /// Computes BM25 parameters like IDF scores and average document length.
    fn compute_bm25_parameters(&mut self, documents: &[String]) {
        let mut doc_lengths = Vec::new();
        let mut term_doc_counts = HashMap::new();

        for doc in documents {
            let terms = Self::tokenize(doc);
            doc_lengths.push(terms.len());

            let unique_terms: HashSet<&str> = terms.iter().copied().collect();
            for term in unique_terms {
                *term_doc_counts.entry(term.to_string()).or_insert(0) += 1;
            }
        }

        self.avg_doc_length = doc_lengths.iter().sum::<usize>() as f32 / documents.len() as f32;

        let total_docs = documents.len() as f32;
        for (term, doc_count) in term_doc_counts {
            let idf = ((total_docs - doc_count as f32 + 0.5) / (doc_count as f32 + 0.5) + 1.0).ln();
            self.idf_scores.insert(term, idf);
        }
    }

    /// Computes the BM25 score for a single document and query.
    fn compute_bm25_score(&self, document: &str, query_terms: &[&str]) -> f32 {
        const K1: f32 = 1.5;
        const B: f32 = 0.75;

        let doc_terms = Self::tokenize(document);
        let doc_length = doc_terms.len() as f32;

        let mut term_frequencies = HashMap::new();
        for term in doc_terms {
            *term_frequencies.entry(term).or_insert(0) += 1;
        }

        let mut score = 0.0;
        for &term in query_terms {
            if let Some(&idf) = self.idf_scores.get(term) {
                let tf = term_frequencies.get(term).copied().unwrap_or(0) as f32;
                let numerator = tf * (K1 + 1.0);
                let denominator = tf + K1 * (1.0 - B + B * doc_length / self.avg_doc_length);
                score += idf * numerator / denominator;
            }
        }

        score
    }

    // Tokenization helper function
    fn tokenize(text: &str) -> Vec<&str> {
        text.split_whitespace().collect()
    }
}

/// Response structure for client-side search requests.
#[derive(Debug, Clone)]
pub struct SearchResponse {
    /// A list of search results.
    pub results: Vec<SearchResult>,
    /// Estimated token usage for the request.
    pub usage: u32,
}
