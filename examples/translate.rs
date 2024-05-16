use std::env;

extern crate pg_rust_client as pg_client;
use pg_client::{client, translate};

#[tokio::main]
async fn main() {
    let key = env::var("PGKEY").expect("PG Api Key");
    let host = env::var("PGHOST").expect("PG Host");

    let clt = client::Client::new(&host, &key).expect("client value");

    let req = translate::Request {
        text: "The rain in Spain stays mainly in the plain".to_string(),
        source_lang: translate::Language::English,
        target_lang: translate::Language::Spanish,
    };

    let result = clt.translate(&req).await.expect("error from toxicity");

    println!("\n\ntranslate response:\n{:?}\n\n", result);
}
