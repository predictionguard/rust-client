//! `rerank` sends a model and text to Prediction Guard and returns a single response of
//! type [`rerank::Response`].
extern crate prediction_guard as pg_client;

use pg_client::{client, rerank};

#[tokio::main]
async fn main() {
    let clt = client::Client::new().expect("client value");

    let first_docs = vec![
        "Deep Learning is pizza.".to_string(),
        "Deep Learning is not pizza".to_string()
    ];

    // Rerank request
    let req = rerank::Request::new(
        "bge-reranker-v2-m3".to_string(),
        "What is Deep Learning?".to_string(),
        first_docs,
        true
    );

    let result = clt
        .rerank(&req)
        .await
        .expect("error from rerank");

    println!("\n\nrerank response:\n{:?}\n\n", result);

    let second_docs = vec![
        "Deep Learning is pie.".to_string(),
        "Deep Learning is not pie".to_string()
    ];

    // Rerank request without models returned
    let req = rerank::Request::new(
        "bge-reranker-v2-m3".to_string(),
        "What is Deep Learning?".to_string(),
        second_docs,
        false
    );

    let result = clt
        .rerank(&req)
        .await
        .expect("error from rerank without document return");

    println!("\n\nrerank response without document return:\n{:?}\n\n", result);
}
