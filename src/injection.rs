//! Data types used for the injection endpoint.
use serde::{Deserialize, Serialize};

/// Path to the injection endpoint.
pub const PATH: &str = "/injection";

/// Request type for the injection endpoint.
#[derive(Debug, Deserialize, Serialize)]
pub struct Request {
    pub(crate) prompt: String,
    pub(crate) detect: bool,
}

impl Request {
    /// Creates a new request for injection detection.
    ///
    /// ## Arguments
    ///
    /// * `prompt` - The text to be analyzed.
    /// * `detect` - Enables detection in the request.
    pub fn new(prompt: String, detect: bool) -> Request {
        Self { prompt, detect }
    }
}

/// Represents an individual check on the injection endpoint.
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Check {
    pub probability: f64,
    pub index: i64,
}

/// Response type for the injection endpoint.
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Response {
    pub id: String,
    pub object: String,
    pub created: String,
    pub checks: Vec<Check>,
}
