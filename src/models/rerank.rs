use serde::{Deserialize, Serialize};

const MAX_DOCUMENTS: usize = 100;

#[derive(Debug, Serialize, Deserialize)]
pub struct RerankResponse {
    #[serde(default)]
    pub object: String,
    pub data: Vec<RerankResult>,
    #[serde(default)]
    pub model: String,
    pub usage: Usage,
}

/// Represents one of the input documents after reranking, including its relevance score
/// and position in the original input array.
///
/// The reranking operation takes a list of documents and returns them ordered by
/// relevance to the query, with scores attached. Each RerankResult corresponds to
/// one of the input documents.
#[derive(Debug, Serialize, Deserialize)]
pub struct RerankResult {
    /// Relevance score from 0.0 to 1.0, where higher scores indicate
    /// greater relevance to the query
    pub relevance_score: f64,
    /// The position this document had in the original input array.
    /// Can be used to map back to the original document ordering.
    pub index: usize,
    /// A copy of the original document text that was scored.
    /// This is the same text that was provided in the input documents array
    /// at position `index`. May be omitted in API responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<String>,
}

impl RerankResult {
    /// Returns true if this result's relevance score exceeds the given threshold
    pub fn is_relevant(&self, threshold: f64) -> bool {
        self.relevance_score >= threshold
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[derive(Default)]
pub enum RerankModel {
    #[serde(rename = "rerank-2")]
    #[default]
    Rerank2,
    #[serde(rename = "rerank-2-lite")]
    Rerank2Lite,
    #[serde(rename = "rerank-lite-1")]
    RerankLite1,
}

impl RerankModel {
    pub const fn max_context_length(&self) -> usize {
        match self {
            Self::Rerank2 => 16000,
            Self::Rerank2Lite | Self::RerankLite1 => 8000,
        }
    }

    pub const fn embedding_size(&self) -> usize {
        match self {
            Self::Rerank2 => 768,
            Self::Rerank2Lite | Self::RerankLite1 => 384,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Usage {
    pub total_tokens: u32,
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum ValidationError {
    #[error("documents cannot be empty")]
    EmptyDocuments,
    #[error("documents cannot contain more than {MAX_DOCUMENTS} items")]
    TooManyDocuments,
}

/// Request to rerank a set of documents based on their relevance to a query.
#[derive(Debug, Serialize)]
pub struct RerankRequest {
    /// The query text to compare documents against
    pub query: String,
    /// The collection of documents to be reranked by relevance to the query.
    /// Maximum 100 documents. Each document will appear exactly once in the
    /// response, but ordered by relevance score.
    #[serde(with = "validate_documents")]
    pub documents: Vec<String>,
    /// The reranking model to use
    pub model: RerankModel,
    /// Optional limit on number of results to return.
    /// If set, only returns the top K most relevant documents.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<usize>,
}

impl RerankRequest {
    pub fn new(
        query: String,
        documents: Vec<String>,
        model: RerankModel,
        top_k: Option<usize>,
    ) -> Result<Self, ValidationError> {
        if documents.is_empty() {
            return Err(ValidationError::EmptyDocuments);
        }
        if documents.len() > MAX_DOCUMENTS {
            return Err(ValidationError::TooManyDocuments);
        }
        Ok(Self {
            query,
            documents,
            model,
            top_k,
        })
    }
}

mod validate_documents {
    use super::MAX_DOCUMENTS;
    use serde::{Serialize, Serializer};

    pub fn serialize<S>(documents: &Vec<String>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if documents.len() > MAX_DOCUMENTS {
            return Err(serde::ser::Error::custom(format!(
                "documents cannot contain more than {} items",
                MAX_DOCUMENTS
            )));
        }
        documents.serialize(serializer)
    }
}
