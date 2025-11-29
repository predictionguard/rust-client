//! Data types used for the translate endpoint.
//!
//! # Deprecation Notice
//!
//! This module is deprecated as of version 0.15.0 because the translate API endpoint
//! is no longer supported by the Prediction Guard API.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Path to the translate endpoint.
#[deprecated(since = "0.15.0", note = "The translate API endpoint is no longer supported")]
pub const PATH: &str = "/translate";

/// Request type used for the translate endpoint.
#[deprecated(since = "0.15.0", note = "The translate API endpoint is no longer supported")]
#[derive(Debug, Deserialize, Serialize)]
pub struct Request {
    pub(crate) text: String,
    #[serde(deserialize_with = "deserialize_language")]
    pub(crate) source_lang: Language,
    #[serde(deserialize_with = "deserialize_language")]
    pub(crate) target_lang: Language,
    pub(crate) use_third_party_engine: bool,
}

impl Request {
    /// Creates a new request for translation.
    ///
    /// ## Arguments
    ///
    /// * `text` - The text to be translated.
    /// * `source_lang` - The language of the text to be translated.
    /// * `target_lang` - The language to translate the text to.
    /// * `use_third_party_engine` - Whether to use third-party translation engines such as OpenAI, DeepL, and Google.
    pub fn new(
        text: String,
        source_lang: Language,
        target_lang: Language,
        use_third_party_engine: bool,
    ) -> Request {
        Self {
            text,
            source_lang,
            target_lang,
            use_third_party_engine,
        }
    }
}

/// Response type used for the translate endpoint.
#[deprecated(since = "0.15.0", note = "The translate API endpoint is no longer supported")]
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Response {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub best_translation: String,
    pub best_score: f64,
    pub best_translation_model: String,
    pub translations: Vec<Translation>,
}

/// Represents an individual translation from the translate endpoint.
#[deprecated(since = "0.15.0", note = "The translate API endpoint is no longer supported")]
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Translation {
    pub score: f64,
    pub translation: String,
    pub model: String,
    pub status: String,
}

/// Languages supported by the translate endpoint.
#[deprecated(since = "0.15.0", note = "The translate API endpoint is no longer supported")]
#[derive(Debug, Deserialize, PartialEq)]
pub enum Language {
    English,
    Other(String),
}

impl Serialize for Language {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Language::English => serializer.serialize_str("eng"),
            Language::Other(s) => serializer.serialize_str(s.as_str()),
        }
    }
}

fn deserialize_language<'de, D>(deserializer: D) -> Result<Language, D::Error>
where
    D: Deserializer<'de>,
{
    let lang: &str = match Deserialize::deserialize(deserializer) {
        Ok(l) => l,
        Err(e) => return Err(e),
    };

    match lang {
        "eng" => Ok(Language::English),
        _ => Ok(Language::Other(lang.to_string())),
    }
}
