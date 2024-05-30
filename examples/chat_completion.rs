//! `chat_completion` sends a prompt to Prediction Guard and returns a single response of
//! type [`completion::ChatResponse`]
extern crate prediction_guard as pg_client;

use pg_client::{client, completion, models};

#[tokio::main]
async fn main() {
    let pg_env = client::PgEnvironment::from_env().expect("env keys");

    let clt = client::Client::new(pg_env).expect("client value");

    let req = completion::ChatRequest {
        model: models::Model::NeuralChat7B,
        messages: vec![completion::Message {
            role: completion::Roles::User,
            content: "How do you feel about the world in general?".to_string(),
        }],
        max_tokens: 1000,
        temperature: 1.1,
    };

    let result = clt
        .generate_chat_completion(&req)
        .await
        .expect("error from generate chat completion");

    println!("\nchat completion response:\n\n {:?}", result);
}
