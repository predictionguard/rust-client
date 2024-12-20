//! `embedding` sends an image base64 encoded and/or text to Prediction Guard and returns a single response of
//! type [`embedding::Response`].
extern crate prediction_guard as pg_client;

use pg_client::embedding::Direction;
use pg_client::{client, embedding, image};

#[tokio::main]
async fn main() {
    let img_str = match image::encode(
        "https://farm4.staticflickr.com/3300/3497460990_11dfb95dd1_z.jpg".to_string(),
    )
    .await
    {
        Ok(s) => Some(s),
        Err(_) => None,
    };

    let clt = client::Client::new().expect("client value");

    // Load the list of models available for completion.
    let models = clt.retrieve_model_list("embedding".to_string()).await.expect("model list");

    assert!(!models.is_empty());

    // Embedding request can contain text and/or an image. The image should be base64 encoded.
    let req = embedding::Request::new(
        models[0].to_string(),
        Some("skyline with a flying horse".to_string()),
        img_str.clone(),
    );

    let result = clt.embedding(&req).await.expect("error from embeddings");

    println!("\n\nembedding response:\n{:?}\n\n", result);

    // Embedding request with truncation on
    let req_truncation = embedding::Request::new(
        models[0].to_string(),
        Some("skyline with a flying horse".to_string()),
        img_str,
    )
    .truncate(Direction::Right);

    let result_truncation = clt
        .embedding(&req_truncation)
        .await
        .expect("error from embeddings with truncation");

    println!(
        "\n\nembedding response with truncation:\n{:?}\n\n",
        result_truncation
    );
}
