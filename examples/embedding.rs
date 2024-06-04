//! `embedding` sends an image base64 encoded or text to Prediction Guard and returns a single response of
//! type [`embedding::Response`].
extern crate prediction_guard as pg_client;

use pg_client::{client, embedding, models};

#[tokio::main]
async fn main() {
    let pg_env = client::PgEnvironment::from_env().expect("env keys");

    let clt = client::Client::new(pg_env).expect("client value");

    // Embedding request can contain text or an image. The image should be base64 encoded.
    let req = embedding::Request::new(
        models::Model::BridgetowerLargeItmMlmItc,
        Some("skyline with a flying horse".to_string()),
        None,
    );

    let result = clt.embedding(&req).await.expect("error from embeddings");

    println!("\n\nembedding response:\n{:?}\n\n", result);
}
