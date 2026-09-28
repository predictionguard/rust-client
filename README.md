## Prediction Guard Rust Client

[![crates.io](https://img.shields.io/crates/v/prediction-guard.svg)](https://crates.io/crates/prediction-guard)

> [!WARNING]
> **This crate is deprecated and no longer maintained.** Some features are broken or missing, and no further updates will be released. Existing versions remain available on crates.io, but you should migrate to an OpenAI-compatible or Anthropic-compatible client as described below.

### Migrating

The Prediction Guard API is compatible with both OpenAI-style and Anthropic-style clients. Use whichever matches the functionality you need, pointed at the Prediction Guard API with your existing API key.

#### OpenAI-compatible (`async-openai`)

```toml
[dependencies]
async-openai = { version = "0.42", features = ["chat-completion"] }
tokio = { version = "1", features = ["full"] }
```

```rust
use async_openai::{
    config::OpenAIConfig,
    types::chat::{ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs},
    Client,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = OpenAIConfig::new()
        .with_api_base("https://api.predictionguard.com")
        .with_api_key(std::env::var("PREDICTIONGUARD_API_KEY")?);
    let client = Client::with_config(config);

    let request = CreateChatCompletionRequestArgs::default()
        .model("<model-name>")
        .messages([ChatCompletionRequestUserMessageArgs::default()
            .content("How do you feel about the world in general?")
            .build()?
            .into()])
        .max_tokens(1000u32)
        .build()?;

    let response = client.chat().create(request).await?;
    println!("{:?}", response.choices[0].message.content);
    Ok(())
}
```

Enable the `async-openai` features for the endpoints you use (for example `embedding` or `completions`).

#### Anthropic-compatible

Any Anthropic Messages API client can be used by setting its base URL to `https://api.predictionguard.com` and its API key to your Prediction Guard API key.

### Docs

For the full list of endpoints and models, see the [Prediction Guard documentation](https://docs.predictionguard.com).

The [API documentation for this crate](https://docs.rs/prediction-guard/latest/) remains available for reference but will not be updated.
