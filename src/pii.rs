//! Data types for the PII detection endpoint.
use serde::{Deserialize, Serialize};

/// Path to the PII endpoint.
pub const PATH: &str = "/PII";

/// Denotes the method to check for PII on inputs for completion and chat completions.
#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub enum InputMethod {
    #[serde(rename = "replace")]
    Replace,
    #[serde(rename = "block")]
    #[default]
    Block,
}

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

/// Request type for the PII detection endpoint.
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Request {
    pub(crate) prompt: String,
    pub(crate) replace: bool,
    pub(crate) replace_method: ReplaceMethod,
}

impl Request {
    /// Creates a new request for PII checks.
    ///
    /// ## Arguments
    ///
    /// * `prompt` - The text to be analyzed.
    /// * `replace_method` - The method for replacing PII information.
    pub fn new(prompt: String, replace: bool, replace_method: ReplaceMethod) -> Request {
        Self {
            prompt,
            replace,
            replace_method,
        }
    }
}

/// Represents individual check from the factuality endpoint.
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Check {
    pub new_prompt: String,
    pub index: i64,
    pub status: String,
}

/// Response type for the PII detection endpoint.
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Response {
    pub id: String,
    pub object: String,
    pub created: String,
    pub checks: Vec<Check>,
}
