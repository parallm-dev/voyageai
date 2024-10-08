use crate::models::RerankModel;
use crate::voyage_errors::RerankBuilderError;
use crate::VoyageAiClient;

#[derive(Debug, Default)]
pub struct RerankRequestBuilder {
    query: Option<String>,
    documents: Option<Vec<String>>,
    model: Option<RerankModel>,
    top_n: Option<usize>,
    truncate: Option<bool>,
    include_metadata: Option<bool>,
    voyage: Option<VoyageAiClient>,
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

    pub fn voyage(mut self, voyage: VoyageAiClient) -> Self {
        self.voyage = Some(voyage);
        self
    }

    pub fn build(self) -> Result<RerankRequest, RerankBuilderError> {
        let query = self.query.ok_or(RerankBuilderError::MissingQuery)?;
        let documents = self.documents.ok_or(RerankBuilderError::MissingDocuments)?;
        let model = self.model.ok_or(RerankBuilderError::MissingModel)?;
        let voyage = self.voyage.ok_or(RerankBuilderError::MissingVoyage)?;

        Ok(RerankRequest {
            query,
            documents,
            model,
            top_n: self.top_n,
            truncate: self.truncate,
            include_metadata: self.include_metadata,
            voyage,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct RerankRequest {
    pub query: String,
    pub documents: Vec<String>,
    pub model: RerankModel,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_n: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncate: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_metadata: Option<bool>,
    #[serde(skip)]
    pub voyage: VoyageAiClient,
}
