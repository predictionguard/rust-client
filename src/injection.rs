use serde::{Deserialize, Serialize};

pub static PATH: &str = "/injection";

#[derive(Debug, Deserialize, Serialize)]
pub struct Check {
    pub probability: f64,
    pub index: i64,
    pub status: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Request {
    pub prompt: String,
    pub detect: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    pub id: Option<String>,
    pub object: Option<String>,
    pub created: Option<String>,
    pub checks: Option<Vec<Check>>,
}
