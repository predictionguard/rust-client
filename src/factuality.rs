//! Data types that are used for the factuality endpoints.
use serde::{Deserialize, Serialize};

/// Path to the factuality endpoint.
pub const PATH: &str = "/factuality";

/// Request type for the factuality endpoint.
#[derive(Debug, Deserialize, Serialize)]
pub struct Request {
    pub(crate) reference: String,
    pub(crate) text: String,
}

impl Request {
    pub fn new(reference: String, text: String) -> Request {
        Self { reference, text }
    }
}
/// Response type for the factuality endpoint.
#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    pub id: Option<String>,
    pub object: Option<String>,
    pub created: Option<i64>,
    pub checks: Option<Vec<Check>>,
}

/// Represents an individual check in the factuality Response.
#[derive(Debug, Deserialize, Serialize)]
pub struct Check {
    pub score: f64,
    pub index: i64,
    pub status: String,
}
