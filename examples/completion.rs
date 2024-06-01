//! `completion` sends a prompt to Prediction Guard and returns a single response of
//! type [`completion::Response`].
extern crate prediction_guard as pg_client;

use pg_client::{client, completion, models};

#[tokio::main]
async fn main() {
    let pg_env = client::PgEnvironment::from_env().expect("env keys");

    let clt = client::Client::new(pg_env).expect("client value");

    let req = completion::Request::new(
        models::Model::NeuralChat7B,
        "Will I lose my hair?".to_string(),
    );

    let result = clt
        .generate_completion(&req)
        .await
        .expect("completion response");

    println!("\ncompletion response:\n\n{:?}", result);
}
