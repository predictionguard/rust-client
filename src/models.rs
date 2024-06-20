//! The models that are available to use.
use serde::{Deserialize, Deserializer, Serialize, Serializer};

const HERMES2_PRO_LLAMA_38B: &str = "Hermes-2-Pro-Llama-3-8B";
const NOUS_HERMES_LLAMA2_13B: &str = "Nous-Hermes-Llama2-13B";
const HERMES_2_PRO_MISTRAL_7B: &str = "Hermes-2-Pro-Mistral-7B";
const NEURAL_CHAT_7B: &str = "Neural-Chat-7B";
const LLAMA_3_SQLCODER_8B: &str = "llama-3-sqlcoder-8b";
const DEEPSEEK_CODER_67B_INSTRUCT: &str = "deepseek-coder-6.7b-instruct";
const BRIDGETOWER_LARGE_ITM_MLM_ITC: &str = "bridgetower-large-itm-mlm-itc";
const LLAVA_15_7B_HF: &str = "llava-1.5-7b-hf";

/// The different models that can be used.
#[derive(Debug, PartialEq, Default, Clone)]
pub enum Model {
    Hermes2ProLlama38B,
    NousHermesLlama213B,
    Hermes2ProMistral7B,
    #[default]
    NeuralChat7B,
    Llama3SqlCoder8b,
    DeepseekCoder67binstruct,
    BridgetowerLargeItmMlmItc,
    Llava157bhf,
    Other(String),
}

impl Serialize for Model {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Model::Hermes2ProLlama38B => serializer.serialize_str(HERMES2_PRO_LLAMA_38B),
            Model::NousHermesLlama213B => serializer.serialize_str(NOUS_HERMES_LLAMA2_13B),
            Model::Hermes2ProMistral7B => serializer.serialize_str(HERMES_2_PRO_MISTRAL_7B),
            Model::NeuralChat7B => serializer.serialize_str(NEURAL_CHAT_7B),
            Model::Llama3SqlCoder8b => serializer.serialize_str(LLAMA_3_SQLCODER_8B),
            Model::DeepseekCoder67binstruct => {
                serializer.serialize_str(DEEPSEEK_CODER_67B_INSTRUCT)
            }
            Model::BridgetowerLargeItmMlmItc => {
                serializer.serialize_str(BRIDGETOWER_LARGE_ITM_MLM_ITC)
            }
            Model::Llava157bhf => serializer.serialize_str(LLAVA_15_7B_HF),
            Model::Other(s) => serializer.serialize_str(s.as_str()),
        }
    }
}

pub fn deserialize_models_option<'de, D>(deserializer: D) -> Result<Option<Model>, D::Error>
where
    D: Deserializer<'de>,
{
    match deserialize_models(deserializer) {
        Ok(x) => Ok(Some(x)),
        Err(e) => Err(e),
    }
}

pub fn deserialize_models_vector<'de, D>(deserializer: D) -> Result<Vec<Model>, D::Error>
where
    D: Deserializer<'de>,
{
    let mdls: Vec<&str> = match Deserialize::deserialize(deserializer) {
        Ok(m) => m,
        Err(e) => return Err(e),
    };

    let mut v = Vec::new();
    for mdl in mdls {
        v.push(match mdl {
            HERMES2_PRO_LLAMA_38B => Model::Hermes2ProLlama38B,
            NOUS_HERMES_LLAMA2_13B => Model::NousHermesLlama213B,
            HERMES_2_PRO_MISTRAL_7B => Model::Hermes2ProMistral7B,
            NEURAL_CHAT_7B => Model::NeuralChat7B,
            LLAMA_3_SQLCODER_8B => Model::Llama3SqlCoder8b,
            DEEPSEEK_CODER_67B_INSTRUCT => Model::DeepseekCoder67binstruct,
            BRIDGETOWER_LARGE_ITM_MLM_ITC => Model::BridgetowerLargeItmMlmItc,
            LLAVA_15_7B_HF => Model::Llava157bhf,
            _ => Model::Other(mdl.to_string()),
        });
    }
    Ok(v)
}

pub fn deserialize_models<'de, D>(deserializer: D) -> Result<Model, D::Error>
where
    D: Deserializer<'de>,
{
    let mdl: &str = match Deserialize::deserialize(deserializer) {
        Ok(m) => m,
        Err(e) => return Err(e),
    };

    match mdl {
        HERMES2_PRO_LLAMA_38B => Ok(Model::Hermes2ProLlama38B),
        NOUS_HERMES_LLAMA2_13B => Ok(Model::NousHermesLlama213B),
        HERMES_2_PRO_MISTRAL_7B => Ok(Model::Hermes2ProMistral7B),
        NEURAL_CHAT_7B => Ok(Model::NeuralChat7B),
        LLAMA_3_SQLCODER_8B => Ok(Model::Llama3SqlCoder8b),
        DEEPSEEK_CODER_67B_INSTRUCT => Ok(Model::DeepseekCoder67binstruct),
        BRIDGETOWER_LARGE_ITM_MLM_ITC => Ok(Model::BridgetowerLargeItmMlmItc),
        LLAVA_15_7B_HF => Ok(Model::Llava157bhf),
        _ => Ok(Model::Other(mdl.to_string())),
    }
}
