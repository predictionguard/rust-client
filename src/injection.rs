//! Data types used for the injection endpoint.
use serde::{Deserialize, Serialize};

/// Path to the injection endpoint.
pub const PATH: &str = "/injection";

/// Represents either a single prompt or multiple prompts.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Prompt {
    Single(String),
    Multiple(Vec<String>),
}

impl From<String> for Prompt {
    fn from(s: String) -> Self {
        Prompt::Single(s)
    }
}

impl From<&str> for Prompt {
    fn from(s: &str) -> Self {
        Prompt::Single(s.to_string())
    }
}

impl From<Vec<String>> for Prompt {
    fn from(v: Vec<String>) -> Self {
        Prompt::Multiple(v)
    }
}

/// Request type for the injection endpoint.
#[derive(Debug, Deserialize, Serialize)]
pub struct Request {
    pub(crate) prompt: Prompt,
    pub(crate) detect: bool,
}

impl Request {
    /// Creates a new request for injection detection.
    ///
    /// ## Arguments
    ///
    /// * `prompt` - The text to be analyzed.
    /// * `detect` - Enables detection in the request.
    pub fn new(prompt: impl Into<Prompt>, detect: bool) -> Request {
        Self { 
            prompt: prompt.into(), 
            detect 
        }
    }

    /// Creates a new request for injection detection.
    ///
    /// ## Arguments
    ///
    /// * `prompt` - The text to be analyzed.
    /// * `detect` - Enables detection in the request.
    pub fn new_batch(prompts: Vec<String>, detect: bool) -> Request {
        Self {
            prompt: Prompt::Multiple(prompts),
            detect
        }
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
    pub created: i64,
    pub checks: Vec<Check>,
}
