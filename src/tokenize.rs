//! Data types that are used for the tokenize endpoint.
use serde::{Deserialize, Serialize};

/// Path to the tokenize endpoint.
pub const PATH: &str = "/tokenize";

/// Request type for the tokenize endpoint.
#[derive(Debug, Deserialize, Serialize)]
pub struct Request {
    pub(crate) model: String,
    pub(crate) input: String,
}

impl Request {
    /// Creates a new request for token generation.
    ///
    /// ## Arguments
    ///
    /// * `model` - The model to use for tokenization.
    /// * `input` - The text to tokenize.
    pub fn new(model: String, input: String) -> Request {
        Self { model, input }
    }
}

/// Represents an individual token in the tokenize response.
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Tokens {
    pub id: i64,
    pub start: i64,
    pub end: i64,
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
    pub tokens: Vec<Tokens>,
}
