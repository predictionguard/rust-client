//! `embedding` sends an image base64 encoded and/or text to Prediction Guard and returns a single response of
//! type [`embedding::Response`].
extern crate prediction_guard as pg_client;

use pg_client::{client, embedding, image, models};

#[tokio::main]
async fn main() {
    let pg_env = client::PgEnvironment::from_env().expect("env keys");

    let img_str = match image::encode(
        "https://farm4.staticflickr.com/3300/3497460990_11dfb95dd1_z.jpg".to_string(),
    )
    .await
    {
        Ok(s) => Some(s),
        Err(_) => None,
    };

    let clt = client::Client::new(pg_env).expect("client value");

    // Embedding request can contain text and/or an image. The image should be base64 encoded.
    let req = embedding::Request::new(
        models::Model::BridgetowerLargeItmMlmItc,
        Some("skyline with a flying horse".to_string()),
        img_str,
    )
    .await;

    let result = clt.embedding(&req).await.expect("error from embeddings");

    println!("\n\nembedding response:\n{:?}\n\n", result);
}
