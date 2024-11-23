//! `completion` sends a prompt to Prediction Guard and returns a single response of
//! type [`completion::Response`].
extern crate prediction_guard as pg_client;

use pg_client::{client, completion};

#[tokio::main]
async fn main() {
    let clt = client::Client::new().expect("client value");

    // Load the list of models available for completion.
    let models = clt.retrieve_model_list("completion".to_string()).await.expect("model list");

    assert!(!models.is_empty());

    let req = completion::Request::new(models[0].clone(), "Will I lose my hair?".to_string())
        .max_tokens(300)
        .temperature(0.1)
        .top_p(0.1)
        .top_k(50);

    let result = clt
        .generate_completion(&req)
        .await
        .expect("completion response");

    println!("\ncompletion response:\n\n{:?}", result);
}
