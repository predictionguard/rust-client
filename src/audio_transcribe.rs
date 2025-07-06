////! Data types that are used for the transcription endpoints.
use serde::{self, Deserialize, Serialize};
use crate::pii;

/// Path to the audio transcription endpoint.
pub const PATH: &str = "/audio/transcribe";

/// Allows to request PII check and Injection check on the inputs in the transcription request.
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct RequestInput {
    block_prompt_injection: bool,
    pii: Option<pii::InputMethod>,
    pii_replace_method: Option<pii::ReplaceMethod>,
}

/// Allows for checking the output of the request for factuality and toxicity.
#[derive(Debug, Deserialize, Serialize)]
pub struct RequestOutput {
    pub factuality: bool,
    pub toxicity: bool,
}

/// Completion request for the base transcription endpoint.
#[derive(Debug, Deserialize, Default, Serialize)]
pub struct Request {
    pub(crate) model: String,
    pub(crate) file: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) timestamp_granularities: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) diarization: Option<bool>,
    pub(crate) input: Option<RequestInput>,
    pub(crate) output: Option<RequestOutput>,
}

impl Request {
    /// Creates a new request for transcription.
    ///
    /// ## Arguments
    ///
    /// * `model` - The model to be used for the request.
    /// * `file` - The file to be used for the transcription request.
    pub fn new(model: String, file: String) -> Self {
        Self {
            model,
            file,
            ..Default::default()
        }
    }

    /// Sets language for the request.
    ///
    /// ## Arguments
    ///
    /// * `lang` - Sets the language of the audio to be transcribed
    pub fn language(mut self, lang: String) -> Request {
        self.language = Some(lang);
        self
    }

    /// Sets prompt for the request.
    ///
    /// ## Arguments
    ///
    /// * `prompt` - Sets the prompt for the transcription
    pub fn prompt(mut self, prompt: String) -> Request {
        self.prompt = Some(prompt);
        self
    }

    /// Sets temperature for the request.
    ///
    /// ## Arguments
    ///
    /// * `temp` - Sets whether to chunk the document
    pub fn temperature(mut self, temp: f64) -> Request {
        self.temperature = Some(temp);
        self
    }

    /// Sets the timestamp granularities for the request.
    ///
    /// ## Arguments
    ///
    /// * `time` - The timestamp granularities to populate for this transcription.
    pub fn timestamp_granularities<S: Into<Vec<String>>>(mut self, time: S) -> Request {
        self.timestamp_granularities = Some(time.into());
        self
    }

    /// Sets diarization for the request.
    ///
    /// ## Arguments
    ///
    /// * `diar` - Sets whether to diarize the transcription
    pub fn diarization(mut self, diar: bool) -> Request {
        self.diarization = Some(diar);
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

/// Represents a word in the base transcription response.
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Words {
    pub text: String,
    pub start: f64,
    pub end: f64,
    pub speaker: Option<String>
}

/// Represents a segment in the base transcription response.
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Segments {
    pub text: String,
    pub start: f64,
    pub end: f64,
    pub speaker: Option<String>
}

/// Completion response for the base transcription endpoint.
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Response {
    pub text: String,
    pub task: Option<String>,
    pub language: Option<String>,
    pub duration: Option<f64>,
    pub words: Option<Vec<Words>>,
    pub segments: Option<Vec<Segments>>,
}
