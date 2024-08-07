//! Data types used for the translate endpoint.
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Path to the translate endpoint.
pub const PATH: &str = "/translate";

/// Request type used for the translate endpoint.
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
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Translation {
    pub score: f64,
    pub translation: String,
    pub model: String,
    pub status: String,
}

/// Languages supported by the translate endpoint.
#[derive(Debug, Deserialize, PartialEq)]
pub enum Language {
    Afrikanns,
    Amharic,
    Arabic,
    Armenian,
    Azerbaijan,
    Basque,
    Belarusian,
    Bengali,
    Bosnian,
    Catalan,
    Chechen,
    Cherokee,
    Chinese,
    Croatian,
    Czech,
    Danish,
    Dutch,
    English,
    Estonian,
    Fijian,
    Filipino,
    Finnish,
    French,
    Galician,
    Georgian,
    German,
    Greek,
    Gujarati,
    Haitian,
    Hebrew,
    Hindi,
    Hungarian,
    Icelandic,
    Indonesian,
    Irish,
    Italian,
    Japanese,
    Kannada,
    Kazakh,
    Korean,
    Latvian,
    Lithuanian,
    Macedonian,
    Malay1,
    Malay2,
    Malayalam,
    Maltese,
    Marathi,
    Nepali,
    Norwegian,
    Persian,
    Polish,
    Portuguese,
    Romanian,
    Russian,
    Samoan,
    Serbian,
    Slovak,
    Slovenian,
    Slavonic,
    Spanish,
    Swahili,
    Swedish,
    Tamil,
    Telugu,
    Thai,
    Turkish,
    Ukrainian,
    Urdu,
    Welsh,
    Vietnamese,
    Other(String),
}

impl Serialize for Language {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Language::Afrikanns => serializer.serialize_str("afr"),
            Language::Amharic => serializer.serialize_str("amh"),
            Language::Arabic => serializer.serialize_str("ara"),
            Language::Armenian => serializer.serialize_str("hye"),
            Language::Azerbaijan => serializer.serialize_str("aze"),
            Language::Basque => serializer.serialize_str("eus"),
            Language::Belarusian => serializer.serialize_str("bel"),
            Language::Bengali => serializer.serialize_str("ben"),
            Language::Bosnian => serializer.serialize_str("bos"),
            Language::Catalan => serializer.serialize_str("cat"),
            Language::Chechen => serializer.serialize_str("che"),
            Language::Cherokee => serializer.serialize_str("chr"),
            Language::Chinese => serializer.serialize_str("zho"),
            Language::Croatian => serializer.serialize_str("hrv"),
            Language::Czech => serializer.serialize_str("ces"),
            Language::Danish => serializer.serialize_str("dan"),
            Language::Dutch => serializer.serialize_str("nld"),
            Language::English => serializer.serialize_str("eng"),
            Language::Estonian => serializer.serialize_str("est"),
            Language::Fijian => serializer.serialize_str("fij"),
            Language::Filipino => serializer.serialize_str("fil"),
            Language::Finnish => serializer.serialize_str("fin"),
            Language::French => serializer.serialize_str("fra"),
            Language::Galician => serializer.serialize_str("glg"),
            Language::Georgian => serializer.serialize_str("kat"),
            Language::German => serializer.serialize_str("deu"),
            Language::Greek => serializer.serialize_str("ell"),
            Language::Gujarati => serializer.serialize_str("guj"),
            Language::Haitian => serializer.serialize_str("hat"),
            Language::Hebrew => serializer.serialize_str("heb"),
            Language::Hindi => serializer.serialize_str("hin"),
            Language::Hungarian => serializer.serialize_str("hun"),
            Language::Icelandic => serializer.serialize_str("isl"),
            Language::Indonesian => serializer.serialize_str("ind"),
            Language::Irish => serializer.serialize_str("gle"),
            Language::Italian => serializer.serialize_str("ita"),
            Language::Japanese => serializer.serialize_str("jpn"),
            Language::Kannada => serializer.serialize_str("kan"),
            Language::Kazakh => serializer.serialize_str("kaz"),
            Language::Korean => serializer.serialize_str("kor"),
            Language::Latvian => serializer.serialize_str("lav"),
            Language::Lithuanian => serializer.serialize_str("lit"),
            Language::Macedonian => serializer.serialize_str("mkd"),
            Language::Malay1 => serializer.serialize_str("msa"),
            Language::Malay2 => serializer.serialize_str("zlm"),
            Language::Malayalam => serializer.serialize_str("mal"),
            Language::Maltese => serializer.serialize_str("mlt"),
            Language::Marathi => serializer.serialize_str("mar"),
            Language::Nepali => serializer.serialize_str("nep"),
            Language::Norwegian => serializer.serialize_str("nor"),
            Language::Persian => serializer.serialize_str("fas"),
            Language::Polish => serializer.serialize_str("pol"),
            Language::Portuguese => serializer.serialize_str("por"),
            Language::Romanian => serializer.serialize_str("ron"),
            Language::Russian => serializer.serialize_str("rus"),
            Language::Samoan => serializer.serialize_str("smo"),
            Language::Serbian => serializer.serialize_str("srp"),
            Language::Slovak => serializer.serialize_str("slk"),
            Language::Slovenian => serializer.serialize_str("slv"),
            Language::Slavonic => serializer.serialize_str("chu"),
            Language::Spanish => serializer.serialize_str("spa"),
            Language::Swahili => serializer.serialize_str("swh"),
            Language::Swedish => serializer.serialize_str("swe"),
            Language::Tamil => serializer.serialize_str("tam"),
            Language::Telugu => serializer.serialize_str("tel"),
            Language::Thai => serializer.serialize_str("tha"),
            Language::Turkish => serializer.serialize_str("tur"),
            Language::Ukrainian => serializer.serialize_str("ukr"),
            Language::Urdu => serializer.serialize_str("urd"),
            Language::Welsh => serializer.serialize_str("cym"),
            Language::Vietnamese => serializer.serialize_str("vie"),
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
        "afr" => Ok(Language::Afrikanns),
        "amh" => Ok(Language::Amharic),
        "ara" => Ok(Language::Arabic),
        "hye" => Ok(Language::Armenian),
        "aze" => Ok(Language::Azerbaijan),
        "eus" => Ok(Language::Basque),
        "bel" => Ok(Language::Belarusian),
        "ben" => Ok(Language::Bengali),
        "bos" => Ok(Language::Bosnian),
        "cat" => Ok(Language::Catalan),
        "che" => Ok(Language::Chechen),
        "chr" => Ok(Language::Cherokee),
        "zho" => Ok(Language::Chinese),
        "hrv" => Ok(Language::Croatian),
        "ces" => Ok(Language::Czech),
        "dan" => Ok(Language::Danish),
        "nld" => Ok(Language::Dutch),
        "eng" => Ok(Language::English),
        "est" => Ok(Language::Estonian),
        "fij" => Ok(Language::Fijian),
        "fil" => Ok(Language::Filipino),
        "fin" => Ok(Language::Finnish),
        "fra" => Ok(Language::French),
        "glg" => Ok(Language::Galician),
        "kat" => Ok(Language::Georgian),
        "deu" => Ok(Language::German),
        "ell" => Ok(Language::Greek),
        "guj" => Ok(Language::Gujarati),
        "hat" => Ok(Language::Haitian),
        "heb" => Ok(Language::Hebrew),
        "hin" => Ok(Language::Hindi),
        "hun" => Ok(Language::Hungarian),
        "isl" => Ok(Language::Icelandic),
        "ind" => Ok(Language::Indonesian),
        "gle" => Ok(Language::Irish),
        "ita" => Ok(Language::Italian),
        "jpn" => Ok(Language::Japanese),
        "kan" => Ok(Language::Kannada),
        "kaz" => Ok(Language::Kazakh),
        "kor" => Ok(Language::Korean),
        "lav" => Ok(Language::Latvian),
        "lit" => Ok(Language::Lithuanian),
        "mkd" => Ok(Language::Macedonian),
        "msa" => Ok(Language::Malay1),
        "zlm" => Ok(Language::Malay2),
        "mal" => Ok(Language::Malayalam),
        "mlt" => Ok(Language::Maltese),
        "mar" => Ok(Language::Marathi),
        "nep" => Ok(Language::Nepali),
        "nor" => Ok(Language::Norwegian),
        "fas" => Ok(Language::Persian),
        "plo" => Ok(Language::Polish),
        "por" => Ok(Language::Portuguese),
        "ron" => Ok(Language::Romanian),
        "rus" => Ok(Language::Russian),
        "smo" => Ok(Language::Samoan),
        "srp" => Ok(Language::Serbian),
        "slk" => Ok(Language::Slovak),
        "slv" => Ok(Language::Slovenian),
        "chu" => Ok(Language::Slavonic),
        "spa" => Ok(Language::Spanish),
        "swh" => Ok(Language::Swahili),
        "swe" => Ok(Language::Swedish),
        "tam" => Ok(Language::Tamil),
        "tel" => Ok(Language::Telugu),
        "tha" => Ok(Language::Thai),
        "tur" => Ok(Language::Turkish),
        "ukr" => Ok(Language::Ukrainian),
        "urd" => Ok(Language::Urdu),
        "cym" => Ok(Language::Welsh),
        "vie" => Ok(Language::Vietnamese),

        _ => Ok(Language::Other(lang.to_string())),
    }
}
