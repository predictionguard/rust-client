//! Data types that are used for the factuality endpoints.
use serde::{Deserialize, Serialize};

/// Path to the factuality endpoint.
pub const PATH: &str = "/models";

/// Request type for the factuality endpoint.
#[derive(Debug, Deserialize, Serialize)]
pub struct Request {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) capability: Option<String>,
}

impl Request {
    /// Creates a new request for listing models.
    ///
    /// ## Arguments
    ///
    /// * `capability` - The capability to sort models by.
    pub fn new(capability: String) -> Request {
        Self { capability: Some(capability) }
    }
}
/// Represents the capabilities for a single model.
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct ModelCapabilities {
    pub chat_completion: bool,
    pub chat_with_image: bool,
    pub completion: bool,
    pub embedding: bool,
    pub embedding_with_image: bool,
    pub tokenize: bool,
}

/// Represents a single model response.
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct ModelData {
    pub id: String,
    pub object: String,
    pub created: String,
    pub owned_by: String,
    pub description: String,
    pub max_context_length: i64,
    pub prompt_format: String,
    pub capabilities: ModelCapabilities,
}

/// Response type for the models endpoint.
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Response {
    pub object: String,
    pub data: Vec<ModelData>,
}
