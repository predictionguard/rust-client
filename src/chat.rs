//! Data types that are used for the chat endpoints, including chat and chat
//! events.
use serde::{self, Deserialize, Serialize};

use crate::{models, pii};

/// Path to the completions chat endpoint.
pub const PATH: &str = "/chat/completions";

const IMAGE_URL_TYPE: &str = "image_url";
const TEXT_TYPE: &str = "text";

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct RequestInput {
    block_prompt_injection: bool,
    pii: Option<String>,
    pii_replace_method: Option<pii::ReplaceMethod>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
struct RequestOutput {
    factuality: bool,
    toxicity: bool,
}

#[derive(Serialize, Default, Clone, Deserialize, Debug)]
struct ImageURL {
    // Currently only base64 encoded image works.
    url: String,
}

#[derive(Serialize, Default, Clone, Deserialize, Debug)]
pub struct Content {
    #[serde(rename = "type")]
    content_type: String,
    text: Option<String>,
    image_url: Option<ImageURL>,
}

#[derive(Serialize, Default, Deserialize, Debug)]
pub struct MessageVision {
    pub role: Roles,
    pub content: Vec<Content>,
}

/// Represents a message in the chat response.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Message {
    pub role: Roles,
    pub content: String,
    pub output: Option<String>,
}

/// Used to send a completion request for chat.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Request<T> {
    #[serde(deserialize_with = "models::deserialize_models")]
    model: models::Model,
    messages: Vec<T>,
    max_tokens: i64,
    temperature: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input: Option<RequestInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output: Option<RequestOutput>,
    pub(crate) stream: bool,
}

impl Request<MessageVision> {
    /// Adds an image as a message.
    ///
    /// # Aurguments
    ///
    /// * `role` - The role of the user sending the message.
    ///
    /// * `prompt` - The text prompt to be sent along with the image.
    ///
    /// * `image_uri` -  data URI for base64 encoded image e.g. data:image/jpeg;base64,<data>
    pub fn add_message(
        mut self,
        role: Roles,
        prompt: String,
        image_uri: String,
    ) -> Request<MessageVision> {
        let img_content = Content {
            content_type: IMAGE_URL_TYPE.to_string(),
            image_url: Some(ImageURL { url: image_uri }),
            ..Default::default()
        };

        let txt_content = Content {
            content_type: TEXT_TYPE.to_string(),
            text: Some(prompt),
            ..Default::default()
        };

        self.messages.push(MessageVision {
            role,
            content: vec![img_content, txt_content],
        });

        self
    }
}

impl Request<Message> {
    pub fn add_message(mut self, role: Roles, prompt: String) -> Request<Message> {
        let m = Message {
            role,
            content: prompt,
            ..Default::default()
        };

        self.messages.push(m);
        self
    }
}

impl<T> Request<T> {
    pub fn new(mdl: models::Model) -> Self {
        Self {
            model: mdl,
            messages: Vec::new(),
            max_tokens: 100,
            temperature: 0.0,
            top_p: None,
            input: None,
            output: None,
            stream: false,
        }
    }

    pub fn with_message(mut self, msg: T) -> Request<T> {
        self.messages.push(msg);
        self
    }

    pub fn max_tokens(mut self, max: i64) -> Request<T> {
        self.max_tokens = max;
        self
    }

    pub fn temperature(mut self, temp: f64) -> Request<T> {
        self.temperature = temp;
        self
    }

    pub fn top_p(mut self, top: f64) -> Request<T> {
        self.top_p = Some(top);
        self
    }

    pub fn input(
        mut self,
        block_prompt_injection: bool,
        pii: Option<(String, pii::ReplaceMethod)>,
    ) -> Request<T> {
        match self.input {
            Some(ref mut x) => {
                // set values on request input
                x.block_prompt_injection = block_prompt_injection;
                if let Some(p) = pii {
                    x.pii = Some(p.0);
                    x.pii_replace_method = Some(p.1);
                }
            }
            None => {
                // create request input
                let mut input = RequestInput {
                    block_prompt_injection,
                    ..Default::default()
                };

                if let Some(p) = pii {
                    input.pii = Some(p.0);
                    input.pii_replace_method = Some(p.1);
                }
                self.input = Some(input);
            }
        }
        self
    }

    pub fn output(mut self, check_factuality: bool, check_toxicity: bool) -> Request<T> {
        match self.output {
            Some(ref mut x) => {
                x.factuality = check_factuality;
                x.toxicity = check_toxicity;
            }
            None => {
                self.output = Some(RequestOutput {
                    toxicity: check_toxicity,
                    factuality: check_factuality,
                })
            }
        };

        self
    }
}

/// Represents a choice in the chat response.
#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseChoice {
    pub message: Message,
    pub index: i64,
    pub status: String,
}

/// Reponse returned from the completion response for chat.
#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    pub id: Option<String>,
    pub object: Option<String>,
    pub created: Option<i64>,
    #[serde(deserialize_with = "models::deserialize_models_option")]
    pub model: Option<models::Model>,
    pub choices: Option<Vec<ResponseChoice>>,
}

/// Represents the content that is streamed in a chat events reponse.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EventsDelta {
    pub content: Option<String>,
}

/// Represents the choices in a chat events response.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChoiceEvents {
    pub generated_text: Option<String>,
    pub index: i64,
    pub logprobs: f64,
    pub finish_reason: Option<String>,
    pub delta: Option<EventsDelta>,
}

/// Completion response returned from the chat events endpoint.
#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseEvents {
    pub id: Option<String>,
    pub object: Option<String>,
    pub created: Option<i64>,
    #[serde(deserialize_with = "models::deserialize_models_option")]
    pub model: Option<models::Model>,
    pub choices: Option<Vec<ChoiceEvents>>,
}

/// The different role types for chat requests/respones.
#[derive(Debug, Deserialize, Serialize, PartialEq, Default)]
pub enum Roles {
    #[serde(rename = "system")]
    System,
    #[serde(rename = "user")]
    #[default]
    User,
    #[serde(rename = "assistant")]
    Assistant,
}

#[cfg(test)]
mod tests {
    use crate::chat;
    use crate::models;
    use crate::pii::ReplaceMethod;

    use super::*;

    const PROMPT: &str = "This is a test";
    const IMAGE_URI: &str = "Image URI";

    #[test]
    fn chat_request() {
        let req = chat::Request::<Message>::new(models::Model::NeuralChat7B)
            .temperature(0.1)
            .max_tokens(1000)
            .top_p(12.6)
            .add_message(Roles::User, PROMPT.to_string())
            .input(true, None)
            .output(true, true);

        assert_eq!(req.model, models::Model::NeuralChat7B);
        assert_eq!(req.temperature, 0.1);
        assert_eq!(req.max_tokens, 1000);
        assert_eq!(req.top_p.expect("Some to_p"), 12.6);

        assert_eq!(req.messages.len(), 1);
        assert_eq!(req.messages[0].content, PROMPT);
        assert_eq!(req.messages[0].role, Roles::User);

        let input = req.input.unwrap();
        assert_eq!(input.block_prompt_injection, true);
        assert!(input.pii.is_none());
        assert!(input.pii_replace_method.is_none());

        let output = req.output.unwrap();
        assert_eq!(output.factuality, true);
        assert_eq!(output.toxicity, true);
    }

    #[test]
    fn chat_request_vision() {
        let req = chat::Request::<MessageVision>::new(models::Model::Llava157bhf)
            .temperature(0.2)
            .max_tokens(2000)
            .top_p(12.1)
            .add_message(Roles::User, PROMPT.to_string(), IMAGE_URI.to_string())
            .input(true, Some((PROMPT.to_string(), ReplaceMethod::Fake)))
            .output(true, true);

        assert_eq!(req.model, models::Model::Llava157bhf);
        assert_eq!(req.temperature, 0.2);
        assert_eq!(req.max_tokens, 2000);
        assert_eq!(req.top_p.expect("Some to_p"), 12.1);

        assert_eq!(req.messages.len(), 1);
        assert_eq!(req.messages[0].role, Roles::User);

        let content = req.messages[0].content.clone();
        assert_eq!(content.len(), 2);

        assert_eq!(content[0].content_type, IMAGE_URL_TYPE);
        assert_eq!(
            content[0].clone().image_url.expect("some image url").url,
            IMAGE_URI
        );

        assert_eq!(content[1].content_type, TEXT_TYPE);
        assert_eq!(content[1].clone().text.expect("text prompt"), PROMPT);

        let input = req.input.unwrap();
        assert_eq!(input.block_prompt_injection, true);
        assert_eq!(input.pii, Some(PROMPT.to_string()));
        assert_eq!(input.pii_replace_method, Some(ReplaceMethod::Fake));

        let output = req.output.unwrap();
        assert_eq!(output.factuality, true);
        assert_eq!(output.toxicity, true);
    }
}
