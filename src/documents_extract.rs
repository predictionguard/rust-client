////! Data types that are used for the document endpoints.
use serde::{self, Deserialize, Serialize};
use crate::pii;

/// Path to the documents extract endpoint.
pub const PATH: &str = "/documents/extract";

/// Allows to request PII check and Injection check on the inputs in the chat request.
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct RequestInput {
    block_prompt_injection: bool,
    pii: Option<pii::InputMethod>,
    pii_replace_method: Option<pii::ReplaceMethod>,
    entity_list: Option<Vec<String>>,
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
    pub(crate) file: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) embed_images: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) output_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) chunk_document: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) chunk_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) enable_ocr: Option<bool>,
    pub(crate) input: Option<RequestInput>,
    pub(crate) output: Option<RequestOutput>,
}

impl Request {
    /// Creates a new request for document extract.
    ///
    /// ## Arguments
    ///
    /// * `file` - The file to be used for the document extract request.
    pub fn new(file: String) -> Self {
        Self {
            file,
            ..Default::default()
        }
    }

    /// Sets embed images for the request.
    ///
    /// ## Arguments
    ///
    /// * `embed` - Sets whether to base64 encode and then embed images into the response
    pub fn embed_images(mut self, embed: bool) -> Request {
        self.embed_images = Some(embed);
        self
    }

    /// Sets output format for the request.
    ///
    /// ## Arguments
    ///
    /// * `output` - Sets the output format for the request
    pub fn output_format(mut self, output: String) -> Request {
        self.output_format = Some(output);
        self
    }

    /// Sets chunk document for the request.
    ///
    /// ## Arguments
    ///
    /// * `chunk_doc` - Sets whether to chunk the document
    pub fn chunk_document(mut self, chunk_doc: bool) -> Request {
        self.chunk_document = Some(chunk_doc);
        self
    }

    /// Sets chunk size for the request.
    ///
    /// ## Arguments
    ///
    /// * `size` - Sets the size of chunks when chunking the document
    pub fn chunk_size(mut self, size: i64) -> Request {
        self.chunk_size = Some(size);
        self
    }

    /// Sets enable ocr for the request.
    ///
    /// ## Arguments
    ///
    /// * `ocr` - Sets whether to enable ocr for the image parsing
    pub fn enable_ocr(mut self, ocr: bool) -> Request {
        self.enable_ocr = Some(ocr);
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

/// Document extract response.
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Response {
    pub title: String,
    pub contents: String,
    pub count: i64,
}
