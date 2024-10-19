use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
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


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SearchType {
    #[serde(rename = "similarity")]
    Similarity,
    #[serde(rename = "nearest_neighbor")]
    NearestNeighbor,
    #[serde(rename = "nearest_duplicate")]
    NearestDuplicate,
    #[serde(rename = "bm25")]
    BM25,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResponse {
    pub object: String,
    pub model: SearchModel,
    pub results: Vec<SearchResult>,
    pub estimated_usage: EstimatedUsage,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchResult {
    pub document: String,
    pub score: f32,
    pub index: usize,
    pub search_type: SearchType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EstimatedUsage {
    pub total_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchQuery {
    pub query: String,
    pub model: SearchModel,
    pub max_results: Option<usize>,
}
use std::cmp::Ordering;

impl PartialOrd for SearchResult {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SearchResult {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.partial_cmp(&other.score).unwrap()
    }
}
