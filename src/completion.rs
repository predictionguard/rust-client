//! Data types that are used for the completion endpoints, including chat and chat
//! events.
use serde::{self, de::Deserializer, Deserialize, Serialize, Serializer};

/// Path to the completions endpoint.
pub const PATH: &str = "/completions";

/// Path to the completions chat endpoint.
pub const CHAT_PATH: &str = "/chat/completions";

/// Used to send a completion request for chat.
#[derive(Debug, Deserialize, Serialize)]
pub struct ChatRequest {
    #[serde(deserialize_with = "deserialize_models")]
    pub model: Models,
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
    #[serde(deserialize_with = "deserialize_models_option")]
    pub model: Option<Models>,
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
    #[serde(deserialize_with = "deserialize_models_option")]
    pub model: Option<Models>,
    pub choices: Option<Vec<ChatChoiceEvents>>,
}

/// Completion request for chat events endpoint.
#[derive(Debug, Deserialize, Serialize)]
pub struct ChatRequestEvents {
    #[serde(deserialize_with = "deserialize_models")]
    pub model: Models,
    pub messages: Vec<Message>,
    pub max_tokens: i64,
    pub temperature: f64,
    pub stream: bool,
}

//********************

/// Completion request for the base completion endpoint.
#[derive(Debug, Deserialize, Serialize)]
pub struct Request {
    #[serde(deserialize_with = "deserialize_models")]
    pub model: Models,
    pub prompt: String,
}

/// Represents a choice in the base completion response.
#[derive(Debug, Deserialize, Serialize)]
pub struct Choice {
    pub text: String,
    pub index: i64,
    pub status: String,
    #[serde(deserialize_with = "deserialize_models")]
    pub model: Models,
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

/// The different models that can be used in the completion endpoints.
#[derive(Debug, PartialEq)]
pub enum Models {
    Hermes2ProLlama38B,
    NousHermesLlama213B,
    Hermes2ProMistral7B,
    NeuralChat7B,
    Yi34BChat,
    DeepseekCoder67binstruct,
    Other(String),
}

impl Serialize for Models {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Models::Hermes2ProLlama38B => serializer.serialize_str("Hermes-2-Pro-Llama-3-8B"),
            Models::NousHermesLlama213B => serializer.serialize_str("Nous-Hermes-Llama2-13B"),
            Models::Hermes2ProMistral7B => serializer.serialize_str("Hermes-2-Pro-Mistral-7B"),
            Models::NeuralChat7B => serializer.serialize_str("Neural-Chat-7B"),
            Models::Yi34BChat => serializer.serialize_str("Yi-34B-Chat"),
            Models::DeepseekCoder67binstruct => {
                serializer.serialize_str("deepseek-coder-6.7b-instruct")
            }
            Models::Other(s) => serializer.serialize_str(s.as_str()),
        }
    }
}

fn deserialize_models_option<'de, D>(deserializer: D) -> Result<Option<Models>, D::Error>
where
    D: Deserializer<'de>,
{
    match deserialize_models(deserializer) {
        Ok(x) => Ok(Some(x)),
        Err(e) => Err(e),
    }
}

fn deserialize_models<'de, D>(deserializer: D) -> Result<Models, D::Error>
where
    D: Deserializer<'de>,
{
    let mdl: &str = match Deserialize::deserialize(deserializer) {
        Ok(m) => m,
        Err(e) => return Err(e),
    };

    match mdl {
        "Hermes-2-Pro-Llama-3-8B" => Ok(Models::Hermes2ProLlama38B),
        "Nous-Hermes-Llama2-13B" => Ok(Models::NousHermesLlama213B),
        "Hermes-2-Pro-Mistral-7B" => Ok(Models::Hermes2ProMistral7B),
        "Neural-Chat-7B" => Ok(Models::NeuralChat7B),
        "Yi-34B-Chat" => Ok(Models::Yi34BChat),
        "deepseek-coder-6.7b-instruct" => Ok(Models::DeepseekCoder67binstruct),
        _ => Ok(Models::Other(mdl.to_string())),
    }
}
