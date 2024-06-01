//! Data types for the PII detection endpoint.
use serde::{Deserialize, Serialize};

/// Path to the PII endpoint.
pub const PATH: &str = "/PII";

/// Denotes the different ways to replace any PII information that is found.
#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub enum ReplaceMethod {
    #[serde(rename = "random")]
    #[default]
    Random,
    #[serde(rename = "mask")]
    Mask,
    #[serde(rename = "category")]
    Category,
    #[serde(rename = "fake")]
    Fake,
}

/// Represents individual check from the factuality endpoint.
#[derive(Debug, Deserialize, Serialize)]
pub struct Check {
    pub new_prompt: String,
    pub index: i64,
    pub status: String,
}

/// Request type for the PII detection endpoint.
#[derive(Debug, Deserialize, Serialize)]
pub struct Request {
    pub(crate) prompt: String,
    pub(crate) replace: bool,
    pub(crate) replace_method: ReplaceMethod,
}

impl Request {
    /// Creates an instance of pii Request.
    pub fn new(prompt: String, replace: bool, replace_method: ReplaceMethod) -> Request {
        Self {
            prompt,
            replace,
            replace_method,
        }
    }
}
/// Response type for the PII detection endpoint.
#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    pub id: Option<String>,
    pub object: Option<String>,
    pub created: Option<String>,
    pub checks: Option<Vec<Check>>,
}
