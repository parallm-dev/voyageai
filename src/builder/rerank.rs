use crate::client::VoyageAiClient;
use crate::errors::VoyageBuilderError;
use crate::models::RerankModel;
use serde::Serialize;

#[derive(Debug, Default)]
pub struct RerankRequestBuilder {
    query: Option<String>,
    documents: Option<Vec<String>>,
    model: Option<RerankModel>,
    top_n: Option<usize>,
    truncate: Option<bool>,
    include_metadata: Option<bool>,
}

impl RerankRequestBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.query = Some(query.into());
        self
    }

    pub fn documents(mut self, documents: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.documents = Some(documents.into_iter().map(Into::into).collect());
        self
    }

    pub fn model(mut self, model: RerankModel) -> Self {
        self.model = Some(model);
        self
    }

    pub fn top_n(mut self, top_n: usize) -> Self {
        self.top_n = Some(top_n);
        self
    }

    pub fn truncate(mut self, truncate: bool) -> Self {
        self.truncate = Some(truncate);
        self
    }

    pub fn include_metadata(mut self, include_metadata: bool) -> Self {
        self.include_metadata = Some(include_metadata);
        self
    }

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
            top_n: self.top_n,
            truncate: self.truncate.unwrap_or(false),
            include_metadata: self.include_metadata.unwrap_or(false),
        })
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct RerankRequest {
    pub query: String,
    pub documents: Vec<String>,
    pub model: RerankModel,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_n: Option<usize>,
    pub truncate: bool,
    pub include_metadata: bool,
}

impl RerankRequest {
    pub fn share(&self) -> RerankRequest {
        self.clone()
    }

    pub fn request(&self) -> &RerankRequest {
        self
    }
}
