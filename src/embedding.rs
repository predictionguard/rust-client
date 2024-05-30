use serde::{Deserialize, Serialize};

use crate::models;

pub(crate) const PATH: &str = "/embeddings";

#[derive(Serialize, Deserialize, Debug)]
pub struct Input {
    pub text: String,
    pub image: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub input: Vec<Input>,
    #[serde(deserialize_with = "models::deserialize_models")]
    pub model: models::Model,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub status: Option<String>,
    pub index: i64,
    pub object: Option<String>,
    pub embedding: Option<Vec<f64>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub id: Option<String>,
    pub object: Option<String>,
    pub created: i64,
    #[serde(deserialize_with = "models::deserialize_models_option")]
    pub model: Option<models::Model>,
    pub data: Option<Vec<Data>>,
}
