//! Data types that are used for the completion endpoints.
use serde::{self, Deserialize, Serialize};

use crate::{models, pii};

/// Path to the completions endpoint.
pub const PATH: &str = "/completions";

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct RequestInput {
    pub(crate) block_prompt_injection: bool,
    pub(crate) pii: Option<String>,
    pub(crate) pii_replace_method: Option<pii::ReplaceMethod>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RequestOutput {
    pub factuality: bool,
    pub toxicity: bool,
}

/// Completion request for the base completion endpoint.
#[derive(Debug, Deserialize, Default, Serialize)]
pub struct Request {
    #[serde(deserialize_with = "models::deserialize_models")]
    pub(crate) model: models::Model,
    pub(crate) prompt: String,
    pub(crate) max_tokens: Option<i64>,
    pub(crate) temperature: Option<f64>,
    pub(crate) top_p: Option<f64>,
    pub(crate) input: Option<RequestInput>,
    pub(crate) output: Option<RequestOutput>,
}

impl Request {
    pub fn new(model: models::Model, prompt: String) -> Self {
        Self {
            model,
            prompt,
            ..Default::default()
        }
    }

    pub fn max_tokens(mut self, max: i64) -> Request {
        self.max_tokens = Some(max);
        self
    }

    pub fn temperature(mut self, temp: f64) -> Request {
        self.temperature = Some(temp);
        self
    }

    pub fn top_p(mut self, top: f64) -> Request {
        self.top_p = Some(top);
        self
    }

    pub fn input(
        mut self,
        block_prompt_injection: bool,
        pii: Option<(String, pii::ReplaceMethod)>,
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
