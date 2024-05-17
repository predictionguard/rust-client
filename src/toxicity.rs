//! Data types for the tocicity endpoint.
use serde::{Deserialize, Serialize};

/// Path to the toxicity endpoint.
pub static PATH: &str = "/toxicity";

/// Represents an individual check from the toxicity endpoint.
#[derive(Debug, Deserialize, Serialize)]
pub struct Check {
    pub score: f64,
    pub index: i64,
    pub status: String,
}

/// Request type for the toxicity endpoint.
#[derive(Debug, Deserialize, Serialize)]
pub struct Request {
    pub text: String,
}

/// Response type for the toxicity endpoint.
#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    pub id: Option<String>,
    pub object: Option<String>,
    pub created: Option<i64>,
    pub checks: Option<Vec<Check>>,
}
