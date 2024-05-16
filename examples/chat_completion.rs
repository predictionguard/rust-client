use std::env;

extern crate pg_rust_client as pg_client;
use pg_client::{client, completion};

#[tokio::main]
async fn main() {
    let key = env::var("PGKEY").expect("PG Api Key");
    let host = env::var("PGHOST").expect("PG Host");

    let clt = client::Client::new(&host, &key).expect("client value");

    let req = completion::ChatRequest {
        model: completion::Models::NeuralChat7B,
        messages: vec![completion::Message {
            role: completion::Roles::User,
            content: "How do you feel about the president?".to_string(),
        }],
        max_tokens: 250,
        temperature: 1.1,
    };

    let result = clt
        .generate_chat_completion(&req)
        .await
        .expect("error from generate chat completion");

    println!("\nchat completion response:\n\n {:?}", result);
}
