use crate::errors::VoyageBuilderError;
use crate::models::rerank::RerankModel;
use serde::Serialize;

/// Builder for creating a rerank request.
#[derive(Debug, Default)]
pub struct RerankRequestBuilder {
    query: Option<String>,
    documents: Option<Vec<String>>,
    model: Option<RerankModel>,
    top_k: Option<usize>,
}

impl RerankRequestBuilder {
    /// Creates a new `RerankRequestBuilder` instance.
    pub fn new() -> Self {
        Default::default()
    }

    /// Sets the query for the rerank request.
    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.query = Some(query.into());
        self
    }

    /// Sets the documents to be reranked.
    pub fn documents<I, T>(mut self, documents: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        self.documents = Some(documents.into_iter().map(Into::into).collect());
        self
    }

    /// Sets the model to be used for reranking.
    pub fn model(mut self, model: RerankModel) -> Self {
        self.model = Some(model);
        self
    }

    /// Sets the number of top results to return.
    pub fn top_k(mut self, top_k: usize) -> Self {
        self.top_k = Some(top_k);
        self
    }

    /// Builds the `RerankRequest` from the builder.
    pub fn build(self) -> Result<RerankRequest, VoyageBuilderError> {
        let query = self
            .query
            .ok_or(VoyageBuilderError::MissingField("query".to_string()))?;
        let documents = self
            .documents
            .ok_or(VoyageBuilderError::MissingField("documents".to_string()))?;
        let model = self.model.ok_or(VoyageBuilderError::MissingModel)?;

        Ok(RerankRequest {
            query,
            documents,
            model,
            top_k: self.top_k,
        })
    }
}

/// Represents a rerank request to be sent to the API.
#[derive(Debug, Serialize, Clone)]
pub struct RerankRequest {
    /// The query to rerank documents against.
    pub query: String,
    /// The documents to be reranked.
    pub documents: Vec<String>,
    /// The model to be used for reranking.
    pub model: RerankModel,
    /// The number of top results to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<usize>,
}

impl RerankRequest {
    /// Creates a shared clone of the `RerankRequest`.
    pub fn share(&self) -> Self {
        self.clone()
    }
}
