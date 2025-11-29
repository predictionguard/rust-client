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

/// Request type for the PII detection endpoint.
#[derive(Debug, Deserialize, Serialize)]
pub struct Request {
    pub(crate) prompt: Prompt,
    pub(crate) replace: bool,
    pub(crate) replace_method: ReplaceMethod,
    pub(crate) entity_list: Vec<String>,
}

impl Request {
    /// Creates a new request for PII checks with a single prompt.
    ///
    /// ## Arguments
    ///
    /// * `prompt` - The text to be analyzed.
    /// * `replace` - Whether to replace any PII present in the prompt.
    /// * `replace_method` - The method for replacing PII information.
    /// * `entity_list` - A vector of entities that the PII checker should ignore
    pub fn new(prompt: impl Into<Prompt>, replace: bool, replace_method: ReplaceMethod, entity_list: Vec<String>) -> Request {
        Self {
            prompt: prompt.into(),
            replace,
            replace_method,
            entity_list,
        }
    }

    /// Creates a new request for PII checks with multiple prompts.
    ///
    /// ## Arguments
    ///
    /// * `prompts` - A vector of texts to be analyzed.
    /// * `replace` - Whether to replace any PII present in the prompts.
    /// * `replace_method` - The method for replacing PII information.
    /// * `entity_list` - A vector of entities that the PII checker should ignore
    pub fn new_batch(prompts: Vec<String>, replace: bool, replace_method: ReplaceMethod, entity_list: Vec<String>) -> Request {
        Self {
            prompt: Prompt::Multiple(prompts),
            replace,
            replace_method,
            entity_list,
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct TypesAndPositions {
    pub start: i64,
    pub end: i64,
    pub r#type: String,
}

/// Represents individual check from the factuality endpoint.
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Check {
    pub prompt: Option<String>,
    pub new_prompt: Option<String>,
    pub types_and_positions: Option<Vec<TypesAndPositions>>,
    pub index: i64,
}

/// Response type for the PII detection endpoint.
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Response {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub checks: Vec<Check>,
}
