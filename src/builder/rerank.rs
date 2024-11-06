use crate::models::rerank::{RerankModel, RerankRequest};

#[derive(Clone)]
pub struct RerankRequestBuilder {
    query: Option<String>,
    documents: Option<Vec<String>>,
    model: Option<RerankModel>,
    top_k: Option<usize>,
}

impl Default for RerankRequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl RerankRequestBuilder {
    pub fn new() -> Self {
        Self {
            query: None,
            documents: None,
            model: None,
            top_k: None,
        }
    }

    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.query = Some(query.into());
        self
    }

    pub fn documents(mut self, documents: Vec<String>) -> Self {
        self.documents = Some(documents);
        self
    }

    pub fn model(mut self, model: RerankModel) -> Self {
        self.model = Some(model);
        self
    }

    pub fn top_k(mut self, top_k: usize) -> Self {
        self.top_k = Some(top_k);
        self
    }

    pub fn build(self) -> Result<RerankRequest, &'static str> {
        let query = self.query.ok_or("Query is required")?;
        let documents = self.documents.ok_or("Documents are required")?;
        let model = self.model.ok_or("Model is required")?;

        Ok(RerankRequest {
            query,
            documents,
            model,
            top_k: self.top_k,
        })
    }
}
