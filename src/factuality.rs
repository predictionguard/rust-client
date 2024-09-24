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
    /// Creates a new request for factuality detection.
    ///
    /// ## Arguments
    ///
    /// * `reference` - The reference text to be used in the factuality check.
    /// * `text` - The text to check for factuality.
    pub fn new(reference: String, text: String) -> Request {
        Self { reference, text }
    }
}
/// Response type for the factuality endpoint.
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Response {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub checks: Vec<Check>,
}

/// Represents an individual check in the factuality Response.
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Check {
    pub score: f64,
    pub index: i64,
}
