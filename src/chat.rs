//! Data types that are used for the chat endpoints, including chat completions, chat vision
//! and chat events.

use std::collections::HashMap;
use serde::{self, Deserialize, Serialize};
use serde_json::Value;
use crate::pii;

/// Path to the completions chat endpoint.
pub const PATH: &str = "/chat/completions";

const IMAGE_URL_TYPE: &str = "image_url";
const TEXT_TYPE: &str = "text";

/// Allows to request PII check and Injection check on the inputs in the chat request.
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct RequestInput {
    block_prompt_injection: bool,
    pii: Option<pii::InputMethod>,
    pii_replace_method: Option<pii::ReplaceMethod>,
}

/// Allows for checking the output of the request for factuality and toxicity.
#[derive(Debug, Deserialize, Serialize, Default)]
pub(crate) struct RequestOutput {
    factuality: bool,
    toxicity: bool,
}

/// Holds a data uri which contains a base64 encoded image.
#[derive(Serialize, Default, Clone, Deserialize, Debug)]
struct ImageURL {
    // Currently only base64 encoded image works.
    url: String,
}

/// Contains the content to use for chat vision. A prompt and an image are specified.
#[derive(Serialize, Default, Clone, Deserialize, Debug)]
pub struct Content {
    #[serde(rename = "type")]
    content_type: String,
    text: Option<String>,
    image_url: Option<ImageURL>,
}

/// Message used when calling chat vision.
#[derive(Serialize, Default, Deserialize, Debug)]
pub struct MessageVision {
    role: Roles,
    content: Vec<Content>,
}

#[derive(Serialize, Default, Deserialize, Debug)]
pub struct Tools {
    #[serde(rename = "type")]
    content_type: Option<String>,
    function: Option<ToolFunction>,
}

#[derive(Serialize, Default, Deserialize, Debug)]
pub struct ToolFunction {
    name: Option<String>,
    description: Option<String>,
    parameters: Option<HashMap<String, Value>>,
    strict: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ToolChoice {
    String(String),
    Structured(ToolChoices),
}

#[derive(Serialize, Default, Deserialize, Debug)]
pub struct ToolChoices {
    #[serde(rename = "type")]
    content_type: Option<String>,
    function: Option<ToolChoiceFunction>,
}

#[derive(Serialize, Default, Deserialize, Debug)]
pub struct ToolChoiceFunction {
    name: Option<String>,
}

impl ToolChoice {
    pub fn auto() -> Self {
        ToolChoice::String("auto".to_string())
    }

    pub fn none() -> Self {
        ToolChoice::String("none".to_string())
    }

    pub fn function(name: impl Into<String>) -> Self {
        ToolChoice::Structured(ToolChoices {
            content_type: Some("function".to_string()),
            function: Some(ToolChoiceFunction {
                name: Some(name.into()),
            }),
        })
    }
}

impl From<String> for ToolChoice {
    fn from(s: String) -> Self {
        ToolChoice::String(s)
    }
}

impl From<&str> for ToolChoice {
    fn from(s: &str) -> Self {
        ToolChoice::String(s.to_string())
    }
}

impl From<ToolChoices> for ToolChoice {
    fn from(tc: ToolChoices) -> Self {
        ToolChoice::Structured(tc)
    }
}

/// Used to send a request for chat.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Request<T> {
    pub(crate) model: String,
    pub(crate) messages: Vec<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) frequency_penalty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) logit_bias: Option<HashMap<i64, i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) max_completion_tokens: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) parallel_tool_calls: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) presence_penalty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) stop: Option<Vec<String>>,
    pub(crate) stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) tool_choice: Option<ToolChoice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) tools: Option<Vec<Tools>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) top_p: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) top_k: Option<i64>,
    pub(crate) input: Option<RequestInput>,
    pub(crate) output: Option<RequestOutput>,
}

impl Request<MessageVision> {
    /// Adds a message to the request when making a chat vision call.
    ///
    /// ## Arguments
    ///
    /// * `role` - The role of the user sending the message.
    /// * `prompt` - The text prompt to be sent along with the image.
    /// * `image_uri` -  data URI for base64 encoded image e.g. data:image/jpeg;base64,data
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
    /// Adds a message to the request when making a chat completion call
    /// or chat events call.
    ///
    /// ## Arguments
    ///
    /// * `role` - The role of the user sending the message.
    /// * `prompt` - The text prompt to be added to the message.
    pub fn add_message(mut self, role: Roles, prompt: String) -> Request<Message> {
        let m = Message {
            role,
            content: prompt,
        };

        self.messages.push(m);
        self
    }
}

impl<T> Request<T> {
    /// Creates a new request for chat.
    ///
    /// ## Arguments
    ///
    /// * `model` - The model to be used for the request.
    pub fn new(model: String) -> Self {
        Self {
            model,
            messages: Vec::new(),
            frequency_penalty: None,
            logit_bias: None,
            max_completion_tokens: None,
            parallel_tool_calls: None,
            presence_penalty: None,
            stop: None,
            tool_choice: None,
            tools: None,
            temperature: None,
            top_p: None,
            top_k: None,
            input: None,
            output: None,
            stream: false,
        }
    }

    /// Returns a request with the added message.
    ///
    /// ## Arguments
    ///
    /// * `msg` - The message to be added to the request.
    pub fn with_message(mut self, msg: T) -> Request<T> {
        self.messages.push(msg);
        self
    }

    /// Returns a request with the list of messages.
    ///
    /// ## Arguments
    ///
    /// * `messages` - The messages to be added to the request.
    pub fn with_messages(mut self, messages: Vec<T>) -> Request<T> {
        for m in messages {
            self.messages.push(m);
        }
        self
    }

    /// Sets the frequency penalty for the request.
    ///
    /// ## Arguments
    ///
    /// * `freq` - A value between -2.0 and 2.0, with positive values increasingly penalizing new tokens based on their frequency so far in order to decrease further occurrences.
    pub fn frequency_penalty(mut self, freq: f64) -> Request<T> {
        self.frequency_penalty = Some(freq);
        self
    }

    /// Sets the logit bias for the request.
    ///
    /// ## Arguments
    ///
    /// * `logit` - Modifies the likelihood of specified tokens appearing in a response.
    pub fn logit_bias(mut self, logit: HashMap<i64, i64>) -> Request<T> {
        self.logit_bias = Some(logit);
        self
    }

    /// Sets the max tokens for the request.
    ///
    /// ## Arguments
    ///
    /// * `max` - The maximum number of tokens to be returned in the response.
    pub fn max_completion_tokens(mut self, max: i64) -> Request<T> {
        self.max_completion_tokens = Some(max);
        self
    }

    /// Sets the max tokens for the request.
    ///
    /// ## Arguments
    ///
    /// * `max` - The maximum number of tokens to be returned in the response.
    #[deprecated(since = "0.15.0")]
    pub fn max_tokens(mut self, max: i64) -> Request<T> {
        self.max_completion_tokens = Some(max);
        self
    }
    
    /// Sets the parallel tool calls for the request.
    ///
    /// ## Arguments
    ///
    /// * `para` - A value between -2.0 and 2.0, with positive values causing a flat reduction of new tokens based on their existing presence so far in order to decrease further occurrences.
    pub fn parallel_tool_calls(mut self, para: bool) -> Request<T> {
        self.parallel_tool_calls = Some(para);
        self
    }

    /// Sets the presence penalty for the request.
    ///
    /// ## Arguments
    ///
    /// * `pres` - A value between -2.0 and 2.0, with positive values causing a flat reduction of new tokens based on their existing presence so far in order to decrease further occurrences.
    pub fn presence_penalty(mut self, pres: f64) -> Request<T> {
        self.presence_penalty = Some(pres);
        self
    }

    /// Sets the stop token for the request.
    ///
    /// ## Arguments
    ///
    /// * `stop` - The token or tokens to stop generation.
    pub fn stop<S: Into<Vec<String>>>(mut self, stop: S) -> Request<T> {
        self.stop = Some(stop.into());
        self
    }

    /// Sets the temperature for the request.
    ///
    /// ## Arguments
    ///
    /// * `temp` - The temperature setting for the request. Used to control randomness.
    pub fn temperature(mut self, temp: f64) -> Request<T> {
        self.temperature = Some(temp);
        self
    }

    /// Sets the tool choice for the request.
    ///
    /// ## Arguments
    ///
    /// * `choice` - The tool for the model to use
    pub fn tool_choice<To: Into<ToolChoice>>(mut self, choice: To) -> Request<T> {
        self.tool_choice = Some(choice.into());
        self
    }

    /// Sets the input parameters for the request, to check for prompt injection and PII.
    ///
    /// ## Arguments
    ///
    /// * `block_prompt_injection` - Determines whether to check for prompt injection in the request.
    /// * `pii` - Sets the `pii::InputMethod` and the `pii::ReplacementMethod`.
    pub fn tools(
        mut self,
        block_prompt_injection: bool,
        pii: Option<(pii::InputMethod, pii::ReplaceMethod)>,
    ) -> Request<T> {
        match self.tools {
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

    /// Sets the Top p for the request.
    ///
    /// ## Arguments
    ///
    /// * `top_p` - The Top p setting for the request. Used to control randomness.
    pub fn top_p(mut self, top_p: f64) -> Request<T> {
        self.top_p = Some(top_p);
        self
    }

    /// Sets the Top k for the request.
    ///
    /// ## Arguments
    ///
    /// * `top_k` - The Top k setting for the request. Used to control randomness.
    pub fn top_k(mut self, top_k: i64) -> Request<T> {
        self.top_k = Some(top_k);
        self
    }

    /// Sets the input parameters for the request, to check for prompt injection and PII.
    ///
    /// ## Arguments
    ///
    /// * `block_prompt_injection` - Determines whether to check for prompt injection in the request.
    /// * `pii` - Sets the `pii::InputMethod` and the `pii::ReplacementMethod`.
    pub fn input(
        mut self,
        block_prompt_injection: bool,
        pii: Option<(pii::InputMethod, pii::ReplaceMethod)>,
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

    /// Sets the output parameters for the request, to check for factuality and toxicity.
    ///
    /// ## Arguments
    ///
    /// * `check_factuality` - Determines whether to check for factuality in the response.
    /// * `check_toxicity` - Determines whether to check for toxicity in the response.
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
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct ResponseChoice {
    pub message: Message,
    pub index: i64,
}

/// Represents a message in the chat response.
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct Message {
    pub role: Roles,
    pub content: String,
}

/// Response returned from the completion response for chat.
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct Response {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<ResponseChoice>,
}

/// Represents the content that is streamed in a chat events response.
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct EventsDelta {
    pub content: String,
}

/// Represents the choices in a chat events response.
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct ChoiceEvents {
    pub generated_text: Option<String>,
    pub index: i64,
    pub finish_reason: Option<String>,
    pub delta: EventsDelta,
}

/// Completion response returned from the chat events endpoint.
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct ResponseEvents {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<ChoiceEvents>,
    pub error: Option<String>,
}

/// The different role types for chat requests/responses.
#[derive(Debug, Deserialize, Serialize, PartialEq, Default, Clone)]
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
    use crate::pii::{InputMethod, ReplaceMethod};

    use super::*;

    const PROMPT: &str = "This is a test";
    const IMAGE_URI: &str = "Image URI";

    #[test]
    fn chat_request() {
        let req = Request::<Message>::new("Hermes-3-Llama-3.1-8B".to_string())
            .temperature(0.1)
            .max_completion_tokens(1000)
            .top_p(12.6)
            .add_message(Roles::User, PROMPT.to_string())
            .input(true, None)
            .output(true, true);

        assert_eq!(req.model, "Hermes-3-Llama-3.1-8B".to_string());
        assert_eq!(req.temperature, Some(0.1));
        assert_eq!(req.max_completion_tokens, Some(1000));
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
        let req = Request::<MessageVision>::new("Qwen2.5-VL-7B-Instruct".to_string())
            .temperature(0.2)
            .max_completion_tokens(2000)
            .top_p(12.1)
            .add_message(Roles::User, PROMPT.to_string(), IMAGE_URI.to_string())
            .input(true, Some((InputMethod::Block, ReplaceMethod::Fake)))
            .output(true, true);

        assert_eq!(req.model, "Qwen2.5-VL-7B-Instruct".to_string());
        assert_eq!(req.temperature, Some(0.2));
        assert_eq!(req.max_completion_tokens, Some(2000));
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
        assert_eq!(input.pii, Some(InputMethod::Block));
        assert_eq!(input.pii_replace_method, Some(ReplaceMethod::Fake));

        let output = req.output.unwrap();
        assert_eq!(output.factuality, true);
        assert_eq!(output.toxicity, true);
    }
}
