//! Data types that are used for the completion endpoints, including chat and chat
//! events.
use serde::{self, Deserialize, Serialize};

use crate::{models, pii};

/// Path to the completions endpoint.
pub const PATH: &str = "/completions";

#[derive(Debug, Deserialize, Serialize)]
pub struct RequestInput {
    pub block_prompt_injection: bool,
    pub pii: String,
    pub pii_replace_method: pii::ReplaceMethod,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RequestOutput {
    pub factuality: bool,
    pub toxicity: bool,
}

/// Completion request for the base completion endpoint.
#[derive(Debug, Deserialize, Default, Serialize)]
pub struct Request {
    #[serde(deserialize_with = "models::deserialize_models_vector")]
    pub model: Vec<models::Model>,
    pub prompt: Vec<String>,
    pub max_tokens: Option<i64>,
    pub temperature: Option<f64>,
    pub top_p: Option<f64>,
    pub input: Option<RequestInput>,
    pub output: Option<RequestOutput>,
}

/// Represents a choice in the base completion response.
#[derive(Debug, Deserialize, Serialize)]
pub struct Choice {
    pub text: String,
    pub index: i64,
    pub status: String,
    #[serde(deserialize_with = "models::deserialize_models")]
    pub model: models::Model,
}

/// Completion response for the base completetion endpoint.
#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    pub id: Option<String>,
    pub object: Option<String>,
    pub created: Option<i64>,
    pub choices: Option<Vec<Choice>>,
}
