//! Data types for the tocicity endpoint.
use serde::{Deserialize, Serialize};

/// Path to the toxicity endpoint.
pub const PATH: &str = "/toxicity";

/// Request type for the toxicity endpoint.
#[derive(Debug, Deserialize, Serialize)]
pub struct Request {
    pub(crate) text: String,
}

impl Request {
    /// Creates a new request for toxicity.
    ///
    /// ## Arguments
    ///
    /// * `text` - The text to be analyzed.
    pub fn new(text: String) -> Request {
        Self { text }
    }
}

/// Represents an individual check from the toxicity endpoint.
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Check {
    pub score: f64,
    pub index: i64,
    pub status: String,
}

/// Response type for the toxicity endpoint.
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Response {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub checks: Vec<Check>,
}
