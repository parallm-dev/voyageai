use crate::errors::VoyageBuilderError;
use crate::models::search::{SearchModel, SearchQuery, SearchType};
use serde::{Deserialize, Serialize};

/// Builder for creating a search request.
#[derive(Debug, Default)]
#[derive(Clone)]
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
        Self::default()
    }

    /// Sets the query for the search request.
    pub fn query(&mut self, query: impl Into<String>) -> &mut Self {
        self.query = Some(query.into());
        self
    }

    /// Sets the documents to be searched.
    pub fn documents<I, T>(&mut self, documents: I) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        self.documents = Some(documents.into_iter().map(Into::into).collect());
        self
    }

    /// Sets the embeddings for the documents.
    pub fn embeddings(&mut self, embeddings: Vec<Vec<f32>>) -> &mut Self {
        self.embeddings = Some(embeddings);
        self
    }

    /// Sets the model to be used for searching.
    pub fn model(&mut self, model: SearchModel) -> &mut Self {
        self.model = Some(model);
        self
    }

    /// Sets the number of top results to return.
    pub fn top_k(&mut self, top_k: usize) -> &mut Self {
        self.top_k = Some(top_k);
        self
    }

    /// Sets the search type.
    pub fn search_type(&mut self, search_type: SearchType) -> &mut Self {
        self.search_type = Some(search_type);
        self
    }

    /// Builds the `SearchRequest` from the builder.
    pub fn build(&self) -> Result<SearchRequest, VoyageBuilderError> {
        let query = self
            .query
            .as_ref()
            .ok_or(VoyageBuilderError::MissingField("query".to_string()))?;
        let model = self.model.clone().ok_or(VoyageBuilderError::MissingModel)?;
        let search_type = self
            .search_type
            .clone()
            .ok_or(VoyageBuilderError::MissingField("search_type".to_string()))?;

        if self.documents.is_none() && self.embeddings.is_none() {
            return Err(VoyageBuilderError::MissingField(
                "documents or embeddings".to_string(),
            ));
        }

        Ok(SearchRequest {
            query: SearchQuery {
                query: query.to_owned(),
                model: SearchModel::default(),
                max_results: None,
                num_results: Some(10),
                include_metadata: Some(false),
            },
            documents: self.documents.clone(),
            embeddings: self.embeddings.clone(),
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

impl SearchRequest {
    /// Creates a shared clone of the `SearchRequest`.
    pub fn share(&self) -> Self {
        self.clone()
    }
}
