use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum SearchModel {
    #[default]
    CosineSimilarity,
    NearestNeighbor,
    BM25,
    NearestDuplicate,
    Custom(String),
}

impl SearchModel {
    pub fn as_str(&self) -> &str {
        match self {
            SearchModel::CosineSimilarity => "cosine_similarity",
            SearchModel::NearestNeighbor => "nearest_neighbor",
            SearchModel::BM25 => "bm25",
            SearchModel::NearestDuplicate => "nearest_duplicate",
            SearchModel::Custom(name) => name,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SearchType {
    #[serde(rename = "similarity")]
    Similarity,
    #[serde(rename = "nearest_neighbor")]
    NearestNeighbor,
    #[serde(rename = "nearest_duplicate")]
    NearestDuplicate,
    #[serde(rename = "bm25")]
    BM25,
    #[serde(rename = "mmr")]
    MaximalMarginalRelevance,
    #[serde(rename = "similarity_score_threshold")]
    SimilarityScoreThreshold,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResponse {
    pub object: String,
    pub model: SearchModel,
    pub results: Vec<SearchResult>,
    pub estimated_usage: EstimatedUsage,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchResult {
    pub document: Vec<String>,
    pub score: i32,
    pub index: usize,
    pub search_type: SearchType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EstimatedUsage {
    pub total_tokens: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchQuery {
    pub query: String,
    pub model: SearchModel,
    pub max_results: Option<usize>,
    pub num_results: Option<usize>,
    pub include_metadata: Option<bool>,
}

impl Default for SearchQuery {
    fn default() -> Self {
        Self {
            query: String::new(),
            model: SearchModel::default(),
            max_results: None,
            num_results: Some(10),
            include_metadata: Some(false),
        }
    }
}

impl From<String> for SearchQuery {
    fn from(query: String) -> Self {
        Self {
            query,
            ..Default::default()
        }
    }
}

use std::cmp::Ordering;

impl PartialOrd for SearchResult {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SearchResult {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.score.cmp(&other.score) {
            ord @ (Ordering::Less | Ordering::Greater) => ord,
            Ordering::Equal => self.index.cmp(&other.index),
        }
    }
}
