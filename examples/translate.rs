//! `translates` sends text, source language and target language to Prediction Guard and returns a single response of
//! type [`translate::Response`].
extern crate prediction_guard as pg_client;

use pg_client::{client, translate};

#[tokio::main]
async fn main() {
    let pg_env = client::PgEnvironment::from_env().expect("env keys");

    let clt = client::Client::new(pg_env).expect("client value");

    let req = translate::Request::new(
        "The rain in Spain stays mainly in the plain".to_string(),
        translate::Language::English,
        translate::Language::Spanish,
        true,
    );

    let result = clt.translate(&req).await.expect("error from translate");

    println!("\n\ntranslate response:\n{:?}\n\n", result);
}
