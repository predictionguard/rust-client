//! Data types used for the injection endpoint.
use serde::{Deserialize, Serialize};

/// Path to the injection endpoint.
pub static PATH: &str = "/injection";

/// Represents an individual check on the injection endpoint.
#[derive(Debug, Deserialize, Serialize)]
pub struct Check {
    pub probability: f64,
    pub index: i64,
    pub status: String,
}

/// Request type for the injection endpoint.
#[derive(Debug, Deserialize, Serialize)]
pub struct Request {
    pub prompt: String,
    pub detect: bool,
}

/// Response type for the injection endpoint.
#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    pub id: Option<String>,
    pub object: Option<String>,
    pub created: Option<String>,
    pub checks: Option<Vec<Check>>,
}
