use std::env;

extern crate prediction_guard as pg_client;
use pg_client::{client, pii};

#[tokio::main]
async fn main() {
    let key = env::var("PGKEY").expect("PG Api Key");
    let host = env::var("PGHOST").expect("PG Host");

    let clt = client::Client::new(&host, &key).expect("client value");

    let req = pii::Request {
        prompt: "My email is joe@gmail.com and my number is 270-123-4567".to_string(),
        replace: true,
        replace_method: pii::ReplaceMethod::Random,
    };

    let result = clt.pii(&req).await.expect("error from pii");

    println!("\n\npii response:\n{:?}\n\n", result);
}
