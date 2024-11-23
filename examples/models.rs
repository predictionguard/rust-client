//! `models` can send a capability to Prediction Guard and returns a single response of
//! type [`models::Response`].
extern crate prediction_guard as pg_client;

use pg_client::{client, models};

#[tokio::main]
async fn main() {
    let clt = client::Client::new().expect("client value");

    // Models request will return all models if set to None
    let req = models::Request::new(
        None,
    );

    let result = clt.models(Some(&req)).await.expect("error from all models");

    println!("\n\nall models response:\n{:?}\n\n", result);

    // Models request will return only models for that capability if set
    let req = models::Request::new(
        Some("chat-completion".to_string()),
    );

    let result = clt.models(Some(&req)).await.expect("error from chat-completion models");

    println!("\n\nchat-completion models response:\n{:?}\n\n", result);
}
