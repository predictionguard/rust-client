use std::env;

extern crate pg_rust_client as pg_client;
use pg_client::{client, factuality};

#[tokio::main]
async fn main() {
    let key = env::var("PGKEY").expect("PG Api Key");
    let host = env::var("PGHOST").expect("PG Host");

    let clt = client::Client::new(&host, &key).expect("client value");

    let req = factuality::Request {
        reference: "The sky is blue".to_string(),
        text: "The sky is green".to_string(),
    };

    let result = clt
        .check_factuality(&req)
        .await
        .expect("error from factuality");

    println!("\n\nfactuality response:\n{:?}", result);
}
