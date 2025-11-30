//! Data types that are used for the detokenize endpoint.
use serde::{Deserialize, Serialize};

/// Path to the detokenize endpoint.
pub const PATH: &str = "/detokenize";

/// Request type for the detokenize endpoint.
#[derive(Debug, Deserialize, Serialize)]
pub struct Request {
    pub(crate) model: String,
    pub(crate) tokens: Vec<i64>,
}

impl Request {
    /// Creates a new request for detokenization.
    ///
    /// ## Arguments
    ///
    /// * `model` - The model to use for detokenization.
    /// * `tokens` - The tokens to detokenize into text.
    pub fn new(model: String, tokens: Vec<i64>) -> Request {
        Self { model, tokens }
    }
}

/// Response type for the detokenize endpoint.
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Response {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub text: String,
}
