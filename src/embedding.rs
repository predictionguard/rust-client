//! Data types to be used in the embedding endpoint.
use serde::{Deserialize, Serialize};

use crate::models;

/// Path to the embedding endpoint.
pub(crate) const PATH: &str = "/embeddings";

/// Input data type to contain either text or a base64 encoded image.
#[derive(Serialize, Default, Deserialize, Debug)]
pub struct Input {
    text: Option<String>,
    image: Option<String>,
}

/// Request data type used for the embedding endpoint.
#[derive(Serialize, Default, Deserialize, Debug)]
pub struct Request {
    pub(crate) input: Vec<Input>,
    #[serde(deserialize_with = "models::deserialize_models")]
    pub(crate) model: models::Model,
}

impl Request {
    /// Creates a new request for embedding. Either text or image is specified.
    /// If both are specified only text will be added.
    ///
    /// ## Arguments
    ///
    /// * `model` - The model to be used for the request.
    /// * `text` - The text used to generate the embedding.
    /// * `image_base64` - A base64 encoded image used to generate the embedding.
    pub fn new(
        model: models::Model,
        text: Option<String>,
        image_base64: Option<String>,
    ) -> Request {
        let mut req = Self {
            model,
            ..Default::default()
        };

        if text.is_some() {
            req.input.push(Input {
                text,
                ..Default::default()
            });
            return req;
        }

        if image_base64.is_some() {
            req.input.push(Input {
                image: image_base64,
                ..Default::default()
            });
        }
        req
    }

    /// Adds input data to the request. Either text or image is specified.
    /// If both are specified only text will be added.
    ///
    /// ## Arguments
    ///
    /// * `text` - The text used to generate the embedding.
    /// * `image_base64` - A base64 encoded image used to generate the embedding.
    pub fn add_input(mut self, text: Option<String>, image_base64: Option<String>) -> Self {
        if text.is_some() {
            self.input.push(Input {
                text,
                ..Default::default()
            });
            return self;
        }

        if image_base64.is_some() {
            self.input.push(Input {
                image: image_base64,
                ..Default::default()
            });
        }
        self
    }
}

/// Contains the embedded data information for the response.
#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub status: Option<String>,
    pub index: i64,
    pub object: Option<String>,
    pub embedding: Option<Vec<f64>>,
}

/// The response returned from the embedding endpoint.
#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub id: Option<String>,
    pub object: Option<String>,
    pub created: i64,
    #[serde(deserialize_with = "models::deserialize_models_option")]
    pub model: Option<models::Model>,
    pub data: Option<Vec<Data>>,
}
