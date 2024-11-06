use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

use crate::builder::search::SearchRequest;
use crate::client::{embeddings_client::EmbeddingClient, rerank_client::RerankClient};
use crate::errors::VoyageError;
use crate::models::search::{SearchResult, SearchType};

// Define SearchRequest struct here

/// Client for performing search operations.
#[derive(Debug, Clone)]
pub struct SearchClient {
    embedding_client: EmbeddingClient,
    #[allow(dead_code)]
    rerank_client: Arc<Box<dyn RerankClient>>,
    #[allow(dead_code)]
    document_index: Arc<Mutex<HashMap<String, Vec<f32>>>>,
    #[allow(dead_code)]
    idf_scores: Arc<Mutex<HashMap<String, f32>>>,
    #[allow(dead_code)]
    avg_doc_length: Arc<Mutex<f32>>,
}

impl SearchClient {

    pub async fn search(&self, request: &SearchRequest) -> Result<Vec<SearchResult>, VoyageError> {
        match request.search_type {
            SearchType::Similarity => self.nearest_neighbor_search(request).await,
            SearchType::NearestNeighbor => self.nearest_neighbor_search(request).await,
            SearchType::BM25 => self.bm25_search(request).await,
            _ => Err(VoyageError::SearchBuilderError(
                "Unsupported search type".to_string(),
            )),
        }
    }

    pub fn new(embedding_client: EmbeddingClient, rerank_client: impl RerankClient + 'static) -> Self {
        Self {
            embedding_client,
            rerank_client: Arc::new(Box::new(rerank_client)),
            document_index: Arc::new(Mutex::new(HashMap::new())),
            idf_scores: Arc::new(Mutex::new(HashMap::new())),
            avg_doc_length: Arc::new(Mutex::new(0.0)),
        }
    }

    // ... (keep existing methods)
    #[allow(dead_code)]
    async fn nearest_neighbor_search(
        &self,
        request: &SearchRequest,
    ) -> Result<Vec<SearchResult>, VoyageError> {
        // Obtain embeddings for the query and documents
        let query_embedding = self.embedding_client.embed(&request.query.query).await?;
        let document_embeddings = match &request.documents {
            Some(docs) => self.embedding_client.embed_batch(docs).await?,
            None => {
                return Err(VoyageError::MissingDocuments(
                    "Missing documents".to_string(),
                ))
            }
        };

        // Calculate distances
        let mut results = request
            .documents
            .as_ref()
            .unwrap()
            .iter()
            .zip(document_embeddings)
            .enumerate()
            .map(|(index, (doc, doc_embedding))| {
                let distance = Self::euclidean_distance(&query_embedding, &doc_embedding);
                SearchResult {
                    document: vec![doc.clone()],
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
    #[allow(dead_code)]
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
        let query_embedding = self.embedding_client.embed(&request.query.query).await?;
        let document_embeddings = self
            .embedding_client
            .embed_batch(request.documents.as_ref().unwrap())
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
                    document: doc.clone(),
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
    async fn bm25_search(&self, request: &SearchRequest) -> Result<Vec<SearchResult>, VoyageError> {
        let documents = request
            .documents
            .as_ref()
            .ok_or_else(|| VoyageError::MissingDocuments("Missing documents".to_string()))?;

        // Ensure the IDF scores and average document length are calculated
        {
            let idf_scores = self.idf_scores.lock().unwrap();
            let avg_doc_length = *self.avg_doc_length.lock().unwrap();
            if idf_scores.is_empty() || avg_doc_length == 0.0 {
                drop(idf_scores);
                self.compute_bm25_parameters(documents);
            }
        }

        // Tokenize the query
        let query_terms = Self::tokenize(&request.query.query);

        // Calculate BM25 scores
        let mut results = documents
            .iter()
            .enumerate()
            .map(|(index, doc)| {
                let score = self.compute_bm25_score(doc, &query_terms);
                SearchResult {
                    document: vec![doc.to_string()],
                    score: score as i32, // Convert to i32 for consistency
                    index,
                    search_type: SearchType::BM25,
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
    fn compute_bm25_parameters(&self, documents: &[String]) {
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

        let avg_doc_length = doc_lengths.iter().sum::<usize>() as f32 / documents.len() as f32;
        *self.avg_doc_length.lock().unwrap() = avg_doc_length;

        let total_docs = documents.len() as f32;
        let mut idf_scores = self.idf_scores.lock().unwrap();
        for (term, doc_count) in term_doc_counts {
            let idf = ((total_docs - doc_count as f32 + 0.5) / (doc_count as f32 + 0.5) + 1.0).ln();
            idf_scores.insert(term, idf);
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

        let idf_scores = self.idf_scores.lock().unwrap();
        let avg_doc_length = *self.avg_doc_length.lock().unwrap();

        let mut score = 0.0;
        for &term in query_terms {
            if let Some(&idf) = idf_scores.get(term) {
                let tf = term_frequencies.get(term).copied().unwrap_or(0) as f32;
                let numerator = tf * (K1 + 1.0);
                let denominator = tf + K1 * (1.0 - B + B * doc_length / avg_doc_length);
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

