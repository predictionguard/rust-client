//! Data types that are used for the completion endpoints.

use std::collections::HashMap;
use serde::{self, Deserialize, Serialize};

use crate::pii;

/// Path to the completions endpoint.
pub const PATH: &str = "/completions";

/// Allows to request PII check and Injection check on the inputs in the chat request.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct RequestInput {
    pub(crate) block_prompt_injection: bool,
    pub(crate) pii: Option<pii::InputMethod>,
    pub(crate) pii_replace_method: Option<pii::ReplaceMethod>,
}

/// Allows for checking the output of the request for factuality and toxicity.
#[derive(Debug, Deserialize, Serialize)]
pub struct RequestOutput {
    pub factuality: bool,
    pub toxicity: bool,
}

/// Completion request for the base completion endpoint.
#[derive(Debug, Deserialize, Default, Serialize)]
pub struct Request {
    pub(crate) model: String,
    pub(crate) prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) frequency_penalty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) logit_bias: Option<HashMap<i64, i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) max_tokens: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) presence_penalty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) stop: Option<Vec<String>>,
    pub(crate) stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) top_p: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) top_k: Option<i64>,
    pub(crate) input: Option<RequestInput>,
    pub(crate) output: Option<RequestOutput>,
}

impl Request {
    /// Creates a new request for completion.
    ///
    /// ## Arguments
    ///
    /// * `model` - The model to be used for the request.
    /// * `prompt` - The prompt to be used for the completion request.
    pub fn new(model: String, prompt: String) -> Self {
        Self {
            model,
            prompt,
            stream: false,
            ..Default::default()
        }
    }

    /// Sets the frequency penalty for the request.
    ///
    /// ## Arguments
    ///
    /// * `freq` - A value between -2.0 and 2.0, with positive values increasingly penalizing new tokens based on their frequency so far in order to decrease further occurrences.
    pub fn frequency_penalty(mut self, freq: f64) -> Request {
        self.frequency_penalty = Some(freq);
        self
    }

    /// Sets the logit bias for the request.
    ///
    /// ## Arguments
    ///
    /// * `logit` - Modifies the likelihood of specified tokens appearing in a response.
    pub fn logit_bias(mut self, logit: HashMap<i64, i64>) -> Request {
        self.logit_bias = Some(logit);
        self
    }

    /// Sets the max tokens for the request.
    ///
    /// ## Arguments
    ///
    /// * `max` - The maximum number of tokens to be returned in the response.
    pub fn max_tokens(mut self, max: i64) -> Request {
        self.max_tokens = Some(max);
        self
    }

    /// Sets the presence penalty for the request.
    ///
    /// ## Arguments
    ///
    /// * `pres` - A value between -2.0 and 2.0, with positive values causing a flat reduction of new tokens based on their existing presence so far in order to decrease further occurrences.
    pub fn presence_penalty(mut self, pres: f64) -> Request {
        self.presence_penalty = Some(pres);
        self
    }

    /// Sets the stop token for the request.
    ///
    /// ## Arguments
    ///
    /// * `stop` - The token or tokens to stop generation.
    pub fn stop<S: Into<Vec<String>>>(mut self, stop: S) -> Request {
        self.stop = Some(stop.into());
        self
    }

    /// Sets the temperature for the request.
    ///
    /// ## Arguments
    ///
    /// * `temp` - The temperature setting for the request. Used to control randomness.
    pub fn temperature(mut self, temp: f64) -> Request {
        self.temperature = Some(temp);
        self
    }

    /// Sets the Top p for the request.
    ///
    /// ## Arguments
    ///
    /// * `top_p` - The Top p setting for the request. Used to control randomness.
    pub fn top_p(mut self, top_p: f64) -> Request {
        self.top_p = Some(top_p);
        self
    }

    /// Sets the Top k for the request.
    ///
    /// ## Arguments
    ///
    /// * `top_k` - The Top k setting for the request. Used to control randomness.
    pub fn top_k(mut self, top_k: i64) -> Request {
        self.top_k = Some(top_k);
        self
    }

    /// Sets the input parameters for the request, to check for prompt injection and PII.
    ///
    /// ## Arguments
    ///
    /// * `block_prompt_injection` - Determines whether to check for prompt injection in
    ///   the request.
    /// * `pii` - Sets the `pii::InputMethod` and the `pii::ReplacementMethod`.
    pub fn input(
        mut self,
        block_prompt_injection: bool,
        pii: Option<(pii::InputMethod, pii::ReplaceMethod)>,
    ) -> Request {
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
    pub fn output(mut self, check_factuality: bool, check_toxicity: bool) -> Request {
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

/// Represents a choice in the base completion response.
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Choice {
    pub text: String,
    pub index: i64,
}

/// Completion response for the base completion endpoint.
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Response {
    pub id: String,
    pub object: String,
    pub model: String,
    pub created: i64,
    pub choices: Vec<Choice>,
}

/// Represents the choices in a completions events response.
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct ChoiceEvents {
    pub text: String,
    pub index: i64,
    pub finish_reason: Option<String>,
    pub stop_reason: Option<String>,
}

/// Completion response returned from the completions events endpoint.
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