//! `tokenize` sends a model and text to Prediction Guard and returns a single response of
//! type [`tokenize::Response`].
extern crate prediction_guard as pg_client;

use pg_client::{client, tokenize};

#[tokio::main]
async fn main() {
    let clt = client::Client::new().expect("client value");

    // Load the list of models available for tokenization.
    let models = clt
        .retrieve_model_list("tokenize".to_string())
        .await
        .expect("model list");

    assert!(!models.is_empty());
    
    let req = tokenize::Request::new(
        models[models.len() - 1].to_string(),
        "Tell me a joke.".to_string(),
    );

    let result = clt
        .tokenize(&req)
        .await
        .expect("error from tokenize");

    println!("\n\ntokenize response:\n{:?}\n\n", result);
}
