//! The models that are available to use.
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// The different models that can be used.
#[derive(Debug, PartialEq, Default, Clone)]
pub enum Model {
    Hermes2ProLlama38B,
    NousHermesLlama213B,
    Hermes2ProMistral7B,
    #[default]
    NeuralChat7B,
    Yi34BChat,
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
            Model::Hermes2ProLlama38B => serializer.serialize_str("Hermes-2-Pro-Llama-3-8B"),
            Model::NousHermesLlama213B => serializer.serialize_str("Nous-Hermes-Llama2-13B"),
            Model::Hermes2ProMistral7B => serializer.serialize_str("Hermes-2-Pro-Mistral-7B"),
            Model::NeuralChat7B => serializer.serialize_str("Neural-Chat-7B"),
            Model::Yi34BChat => serializer.serialize_str("Yi-34B-Chat"),
            Model::DeepseekCoder67binstruct => {
                serializer.serialize_str("deepseek-coder-6.7b-instruct")
            }
            Model::BridgetowerLargeItmMlmItc => {
                serializer.serialize_str("bridgetower-large-itm-mlm-itc")
            }
            Model::Llava157bhf => serializer.serialize_str("llava-1.5-7b-hf"),
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
            "Hermes-2-Pro-Llama-3-8B" => Model::Hermes2ProLlama38B,
            "Nous-Hermes-Llama2-13B" => Model::NousHermesLlama213B,
            "Hermes-2-Pro-Mistral-7B" => Model::Hermes2ProMistral7B,
            "Neural-Chat-7B" => Model::NeuralChat7B,
            "Yi-34B-Chat" => Model::Yi34BChat,
            "deepseek-coder-6.7b-instruct" => Model::DeepseekCoder67binstruct,
            "bridgetower-large-itm-mlm-itc" => Model::BridgetowerLargeItmMlmItc,
            "llava-1.5-7b-hf" => Model::Llava157bhf,
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
        "Hermes-2-Pro-Llama-3-8B" => Ok(Model::Hermes2ProLlama38B),
        "Nous-Hermes-Llama2-13B" => Ok(Model::NousHermesLlama213B),
        "Hermes-2-Pro-Mistral-7B" => Ok(Model::Hermes2ProMistral7B),
        "Neural-Chat-7B" => Ok(Model::NeuralChat7B),
        "Yi-34B-Chat" => Ok(Model::Yi34BChat),
        "deepseek-coder-6.7b-instruct" => Ok(Model::DeepseekCoder67binstruct),
        "bridgetower-large-itm-mlm-itc" => Ok(Model::BridgetowerLargeItmMlmItc),
        "llava-1.5-7b-hf" => Ok(Model::Llava157bhf),
        _ => Ok(Model::Other(mdl.to_string())),
    }
}
