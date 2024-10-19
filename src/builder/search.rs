use crate::errors::VoyageBuilderError;
use crate::models::search::{SearchModel, SearchQuery};
use serde::{Serialize, Deserialize};

/// Builder for creating a search request.
#[derive(Debug, Default)]
pub struct SearchRequestBuilder {
    query: Option<String>,
    documents: Option<Vec<String>>,
    embeddings: Option<Vec<Vec<f32>>>,
    model: Option<SearchModel>,
    top_k: Option<usize>,
    search_type: Option<SearchType>,
}

impl SearchRequestBuilder {
    /// Creates a new `SearchRequestBuilder` instance.
    pub fn new() -> Self {
        Default::default()
    }

    /// Sets the query for the search request.
    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.query = Some(query.into());
        self
    }

    /// Sets the documents to be searched.
    pub fn documents<I, T>(mut self, documents: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        self.documents = Some(documents.into_iter().map(Into::into).collect());
        self
    }

    /// Sets the embeddings for the documents.
    pub fn embeddings(mut self, embeddings: Vec<Vec<f32>>) -> Self {
        self.embeddings = Some(embeddings);
        self
    }

    /// Sets the model to be used for searching.
    pub fn model(mut self, model: SearchModel) -> Self {
        self.model = Some(model);
        self
    }

    /// Sets the number of top results to return.
    pub fn top_k(mut self, top_k: usize) -> Self {
        self.top_k = Some(top_k);
        self
    }

    /// Sets the search type.
    pub fn search_type(mut self, search_type: SearchType) -> Self {
        self.search_type = Some(search_type);
        self
    }

    /// Builds the `SearchRequest` from the builder.
    pub fn build(self) -> Result<SearchRequest, VoyageBuilderError> {
        let query = self.query
            .ok_or(VoyageBuilderError::MissingField("query".to_string()))?;
        let model = self.model.ok_or(VoyageBuilderError::MissingModel)?;
        let search_type = self.search_type
            .ok_or(VoyageBuilderError::MissingField("search_type".to_string()))?;

        if self.documents.is_none() && self.embeddings.is_none() {
            return Err(VoyageBuilderError::MissingField(
                "documents or embeddings".to_string(),
            ));
        }

        Ok(SearchRequest {
            query: SearchQuery {
                query,
                model: model.clone(),
                max_results: self.top_k,
            },
            documents: self.documents,
            embeddings: self.embeddings,
            model,
            top_k: self.top_k,
            search_type,
        })
    }
}

/// Represents a search request to be sent to the API.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchRequest {
    /// The query to search against.
    pub query: SearchQuery,
    /// The documents to be searched (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents: Option<Vec<String>>,
    /// The embeddings of the documents (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeddings: Option<Vec<Vec<f32>>>,
    /// The model to be used for searching.
    pub model: SearchModel,
    /// The number of top results to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<usize>,
    /// The type of search to perform.
    pub search_type: SearchType,
}

/// Enum representing different types of search.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SearchType {
    #[serde(rename = "similarity")]
    Similarity,
    #[serde(rename = "mmr")]
    MaximalMarginalRelevance,
    #[serde(rename = "similarity_score_threshold")]
    SimilarityScoreThreshold,
}

impl SearchRequest {
    /// Creates a shared clone of the `SearchRequest`.
    pub fn share(&self) -> Self {
        self.clone()
    }
}
