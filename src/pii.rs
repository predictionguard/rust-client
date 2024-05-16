use serde::{Deserialize, Serialize};

pub static PATH: &str = "/PII";

#[derive(Debug, Serialize, Deserialize)]
pub enum ReplaceMethod {
    #[serde(rename = "random")]
    Random,
    #[serde(rename = "mask")]
    Mask,
    #[serde(rename = "category")]
    Category,
    #[serde(rename = "fake")]
    Fake,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Check {
    pub new_prompt: String,
    pub index: i64,
    pub status: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Request {
    pub prompt: String,
    pub replace: bool,
    pub replace_method: ReplaceMethod,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    pub id: Option<String>,
    pub object: Option<String>,
    pub created: Option<String>,
    pub checks: Option<Vec<Check>>,
}
