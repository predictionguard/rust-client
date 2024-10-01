//! `toxicity` sends a text prompt to Prediction Guard and returns a single response of
//! type [`toxicity::Response`].
extern crate prediction_guard as pg_client;

use pg_client::{client, toxicity};

#[tokio::main]
async fn main() {
    let clt = client::Client::new().expect("client value");

    let req = toxicity::Request::new(
        "Every flight I have is late and I am very angry. I want to hurt someone.".to_string(),
    );

    let result = clt.toxicity(&req).await.expect("error from toxicity");

    println!("\n\ntoxicity response:\n{:?}\n\n", result);
}
