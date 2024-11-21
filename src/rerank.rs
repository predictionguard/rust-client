//! Data types that are used for the rerank endpoint.
use serde::{Deserialize, Serialize};

/// Path to the rerank endpoint.
pub const PATH: &str = "/rerank";

/// Request type for the tokenize endpoint.
#[derive(Debug, Deserialize, Serialize)]
pub struct Request {
    pub(crate) model: String,
    pub(crate) query: String,
    pub(crate) documents: Vec<String>,
    pub(crate) return_documents: bool,
}

impl Request {
    /// Creates a new request for reranking.
    ///
    /// ## Arguments
    ///
    /// * `model` - The model to use for reranking.
    /// * `query` - The query to rank against.
    /// * `documents` - The documents to rank.
    /// * `return_documents` - Bool for returning documents with scores.
    pub fn new(model: String, query: String, documents: Vec<String>, return_documents: bool) -> Request {
        Self { model, query, documents, return_documents }
    }
}

/// Represents an individual rank in the rerank response.
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Results {
    pub index: i64,
    pub relevance_score: f64,
    pub text: String,
}

/// Response type for the tokenize endpoint.
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Response {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub results: Vec<Results>,
}
