//! `detokenize` sends a model and tokens to Prediction Guard and returns a single response of
//! type [`detokenize::Response`].
extern crate prediction_guard as pg_client;

use pg_client::{client, detokenize};

#[tokio::main]
async fn main() {
    let clt = client::Client::new().expect("client value");

    // Load the list of models available for detokenization.
    let models = clt
        .retrieve_model_list("detokenize".to_string())
        .await
        .expect("model list");

    assert!(!models.is_empty());

    let req = detokenize::Request::new(
        models[models.len() - 1].to_string(),
        // TODO: Change to actual
        vec![],
    );

    let result = clt
        .detokenize(&req)
        .await
        .expect("error from detokenize");

    println!("\n\ndetokenize response:\n{:?}\n\n", result);
}
