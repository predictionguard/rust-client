//! Data types to be used in the embedding endpoint.
use serde::{Deserialize, Serialize};

/// Path to the embedding endpoint.
pub(crate) const PATH: &str = "/embeddings";

#[derive(Serialize, Clone, Default, Deserialize, Debug)]
pub enum Direction {
    #[serde(rename = "Right")]
    #[default]
    Right,
    #[serde(rename = "Left")]
    Left,
}

/// Input data type to contain text and/or a base64 encoded image.
#[derive(Serialize, Clone, Default, Deserialize, Debug)]
pub struct Input {
    pub text: Option<String>,
    pub image: Option<String>,
}

/// Request data type used for the embedding endpoint.
#[derive(Serialize, Clone, Default, Deserialize, Debug)]
pub struct Request {
    pub(crate) input: Vec<Input>,
    pub(crate) model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) truncate: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) truncate_direction: Option<Direction>,
}

impl Request {
    /// Creates a new request for embedding. Text and/or an image is specified.
    ///
    /// ## Arguments
    ///
    /// * `model` - The model to be used for the request.
    /// * `text` - The text used to generate the embedding.
    /// * `image` - A base64 encoded image used to generate the embedding.
    pub fn new(model: String, text: Option<String>, image: Option<String>) -> Request {
        Self {
            model,
            input: vec![Input { text, image }],
            truncate: None,
            truncate_direction: None,
        }
    }

    /// Sets the truncate parameter and the truncate direction on the request.
    ///
    /// ## Arguments
    ///
    /// * `truncate_direction` - The enum value of the direction to truncate the embeddings.
    pub fn truncate(mut self, direction: Direction) -> Self {
        self.truncate = Some(true);
        self.truncate_direction = Some(direction);
        self
    }

    /// Adds input data to the request.
    ///
    /// ## Arguments
    ///
    /// * `text` - The text used to generate the embedding.
    /// * `image` - A base64 encoded image used to generate the embedding.
    pub fn add_input(mut self, text: Option<String>, image: Option<String>) -> Self {
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
    pub model: String,
    pub data: Vec<Data>,
}
