//! Data types to be used in the embedding endpoint.
use serde::{Deserialize, Serialize};

use crate::models;

/// Path to the embedding endpoint.
pub(crate) const PATH: &str = "/embeddings";

/// Input data type to contain text and/or a base64 encoded image.
#[derive(Serialize, Clone, Default, Deserialize, Debug)]
pub struct Input {
    text: Option<String>,
    image: Option<String>,
}

/// Request data type used for the embedding endpoint.
#[derive(Serialize, Clone, Default, Deserialize, Debug)]
pub struct Request {
    pub(crate) input: Vec<Input>,
    #[serde(deserialize_with = "models::deserialize_models")]
    pub(crate) model: models::Model,
}

impl Request {
    /// Creates a new request for embedding. Text and/or an image is specified.
    ///
    /// ## Arguments
    ///
    /// * `model` - The model to be used for the request.
    /// * `text` - The text used to generate the embedding.
    /// * `image` - A base64 encoded image used to generate the embedding.
    pub async fn new(model: models::Model, text: Option<String>, image: Option<String>) -> Request {
        Self {
            model,
            input: vec![Input { text, image }],
        }
    }

    /// Adds input data to the request.
    ///
    /// ## Arguments
    ///
    /// * `text` - The text used to generate the embedding.
    /// * `image` - A base64 encoded image used to generate the embedding.
    pub async fn add_input(mut self, text: Option<String>, image: Option<String>) -> Self {
        self.input.push(Input { text, image });

        self
    }

    /// Adds a list of inputs to the request.
    ///
    /// ## Arguments
    ///
    /// * `inputs` - A list of inputs to add.
    pub fn add_inputs(mut self, inputs: Vec<Input>) -> Self {
        for i in inputs {
            self.input.push(i);
        }
        self
    }
}

/// Contains the embedded data information for the response.
#[derive(Serialize, Default, Deserialize, Debug)]
#[serde(default)]
pub struct Data {
    pub index: i64,
    pub object: String,
    pub embedding: Vec<f64>,
}

/// The response returned from the embedding endpoint.
#[derive(Serialize, Default, Deserialize, Debug)]
#[serde(default)]
pub struct Response {
    pub id: String,
    pub object: String,
    pub created: i64,
    #[serde(deserialize_with = "models::deserialize_models")]
    pub model: models::Model,
    pub data: Vec<Data>,
}
