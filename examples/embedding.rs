//! `embedding` sends an image base64 encoded and text to Prediction Guard and returns a single response of
//! type [`embedding::Response`].
extern crate prediction_guard as pg_client;

use pg_client::{client, embedding, models};

#[tokio::main]
async fn main() {
    let pg_env = client::PgEnvironment::from_env().expect("env keys");

    let file = std::fs::read_to_string("image.txt").expect("text file");

    let clt = client::Client::new(pg_env).expect("client value");

    let input = embedding::Input {
        text: "skyline with a flying horse".to_string(),
        image: file,
    };

    let inputs = vec![input];

    let req = embedding::Request {
        model: models::Model::BridgetowerLargeItmMlmItc,
        input: inputs,
    };

    let result = clt.embedding(&req).await.expect("error from embeddings");

    println!("\n\nembedding response:\n{:?}\n\n", result);
}
