//! Data types that are used for the completion endpoints, including chat and chat
//! events.
use crate::models;
use serde::{self, Deserialize, Serialize};

/// Path to the completions endpoint.
pub const PATH: &str = "/completions";

/// Path to the completions chat endpoint.
pub const CHAT_PATH: &str = "/chat/completions";

/// Used to send a completion request for chat.
#[derive(Debug, Deserialize, Serialize)]
pub struct ChatRequest {
    #[serde(deserialize_with = "models::deserialize_models")]
    pub model: models::Model,
    pub messages: Vec<Message>,
    pub max_tokens: i64,
    pub temperature: f64,
}

/// Represents a message in the chat response.
#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    pub role: Roles,
    pub content: String,
}

/// Represents a choice in the chat response.
#[derive(Debug, Deserialize, Serialize)]
pub struct ChatChoice {
    pub message: Message,
    pub index: i64,
}

/// Reponse returned from the completion response for chat.
#[derive(Debug, Deserialize, Serialize)]
pub struct ChatResponse {
    pub id: Option<String>,
    pub object: Option<String>,
    pub created: Option<i64>,
    #[serde(deserialize_with = "models::deserialize_models_option")]
    pub model: Option<models::Model>,
    pub choices: Option<Vec<ChatChoice>>,
}

//********************

/// Represents the content that is streamed in a chat events reponse.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChatEventsDelta {
    pub content: Option<String>,
}

/// Represents the choices in a chat events response.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChatChoiceEvents {
    pub generated_text: Option<String>,
    pub index: i64,
    pub logprobs: f64,
    pub finish_reason: Option<String>,
    pub delta: Option<ChatEventsDelta>,
}

/// Completion response returned from the chat events endpoint.
#[derive(Debug, Deserialize, Serialize)]
pub struct ChatResponseEvents {
    pub id: Option<String>,
    pub object: Option<String>,
    pub created: Option<i64>,
    #[serde(deserialize_with = "models::deserialize_models_option")]
    pub model: Option<models::Model>,
    pub choices: Option<Vec<ChatChoiceEvents>>,
}

/// Completion request for chat events endpoint.
#[derive(Debug, Deserialize, Serialize)]
pub struct ChatRequestEvents {
    #[serde(deserialize_with = "models::deserialize_models")]
    pub model: models::Model,
    pub messages: Vec<Message>,
    pub max_tokens: i64,
    pub temperature: f64,
    pub stream: bool,
}

//********************

/// Completion request for the base completion endpoint.
#[derive(Debug, Deserialize, Serialize)]
pub struct Request {
    #[serde(deserialize_with = "models::deserialize_models")]
    pub model: models::Model,
    pub prompt: String,
}

/// Represents a choice in the base completion response.
#[derive(Debug, Deserialize, Serialize)]
pub struct Choice {
    pub text: String,
    pub index: i64,
    pub status: String,
    #[serde(deserialize_with = "models::deserialize_models")]
    pub model: models::Model,
}

/// Completion response for the base completetion endpoint.
#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    pub id: Option<String>,
    pub object: Option<String>,
    pub created: Option<i64>,
    pub choices: Option<Vec<Choice>>,
}

/// The different role types for chat requests/respones.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum Roles {
    #[serde(rename = "system")]
    System,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
}
