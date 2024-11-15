//! `models` can send a capability to Prediction Guard and returns a single response of
//! type [`models::Response`].
extern crate prediction_guard as pg_client;

use pg_client::{client, models};

#[tokio::main]
async fn main() {
    let clt = client::Client::new().expect("client value");

    let result = clt
        .retrieve_models()
        .await
        .expect("error from factuality");

    println!("\n\nfactuality response:\n{:?}\n\n", result);
}
