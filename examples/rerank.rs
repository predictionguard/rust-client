//! `rerank` sends a model and text to Prediction Guard and returns a single response of
//! type [`rerank::Response`].
extern crate prediction_guard as pg_client;

use pg_client::{client, rerank};

#[tokio::main]
async fn main() {
    let clt = client::Client::new().expect("client value");

    let docs = vec![
        "Deep Learning is pizza.".to_string(),
        "Deep Learning is not pizza".to_string()
    ];

    let req = rerank::Request::new(
        "bge-reranker-v2-m3".to_string(),
        "What is Deep Learning?".to_string(),
        docs,
        true
    );

    let result = clt
        .rerank(&req)
        .await
        .expect("error from rerank");

    println!("\n\nrerank response:\n{:?}\n\n", result);
}
