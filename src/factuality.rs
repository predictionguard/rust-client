use serde::{Deserialize, Serialize};

pub static PATH: &str = "/factuality";

#[derive(Debug, Deserialize, Serialize)]
pub struct Request {
    pub reference: String,
    pub text: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    pub id: Option<String>,
    pub object: Option<String>,
    pub created: Option<i64>,
    pub checks: Option<Vec<Check>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Check {
    pub score: f64,
    pub index: i64,
    pub status: String,
}
