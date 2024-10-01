//! `chat_completion` sends a prompt to Prediction Guard and returns a single response of
//! type [`chat::Response`]
extern crate prediction_guard as pg_client;

use pg_client::{chat, client};

#[tokio::main]
async fn main() {
    let clt = client::Client::new().expect("client value");

    // Load the list of models available for chat completion.
    let models = clt
        .retrieve_chat_completion_models()
        .await
        .expect("model list");

    assert!(!models.is_empty());

    // use last moddel returned in the list.
    let req = chat::Request::<chat::Message>::new(models[models.len() - 1].to_string())
        .add_message(
            chat::Roles::User,
            "How do you feel about the world in general?".to_string(),
        )
        .max_tokens(1000)
        .temperature(0.1)
        .top_p(0.1)
        .top_k(50);

    let result = clt
        .generate_chat_completion(&req)
        .await
        .expect("error from generate chat completion");

    println!("\nchat completion response:\n\n {:?}", result);
}
