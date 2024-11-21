//! `models` can send a capability to Prediction Guard and returns a single response of
//! type [`models::Response`].
extern crate prediction_guard as pg_client;

use pg_client::{client, models};

#[tokio::main]
async fn main() {
    let clt = client::Client::new().expect("client value");

    let req = models::Request::new(
        Some("chat-completion".to_string()),
    );

    let result = clt.models(Some(&req)).await.expect("error from models");

    println!("\n\nmodels response:\n{:?}\n\n", result);
}
