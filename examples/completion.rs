use std::env;

extern crate prediction_guard as pg_client;
use pg_client::{client, completion};

#[tokio::main]
async fn main() {
    let key = env::var("PGKEY").expect("PG Api Key");
    let host = env::var("PGHOST").expect("PG Host");

    let clt = client::Client::new(&host, &key).expect("client value");

    let req = completion::Request {
        model: completion::Models::NeuralChat7B,
        prompt: "Will I lose my hair?".to_string(),
    };

    let result = clt
        .generate_completion(&req)
        .await
        .expect("completion response");

    println!("\ncompletion response:\n\n{:?}", result);
}
