use serde::{Deserialize, Serialize};

use crate::models;

pub(crate) const PATH: &str = "/embeddings";

#[derive(Serialize, Default, Deserialize, Debug)]
pub struct Input {
    text: Option<String>,
    image: Option<String>,
}

#[derive(Serialize, Default, Deserialize, Debug)]
pub struct Request {
    pub input: Vec<Input>,
    #[serde(deserialize_with = "models::deserialize_models")]
    pub model: models::Model,
}

impl Request {
    pub fn new(
        model: models::Model,
        text: Option<String>,
        image_base64: Option<String>,
    ) -> Request {
        let mut req = Self {
            model,
            ..Default::default()
        };

        if !text.is_none() {
            req.input.push(Input {
                text,
                ..Default::default()
            });
        }

        if !image_base64.is_none() {
            req.input.push(Input {
                image: image_base64,
                ..Default::default()
            });
        }
        req
    }
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
