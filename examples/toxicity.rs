use std::env;

extern crate prediction_guard as pg_client;
use pg_client::{client, toxicity};

#[tokio::main]
async fn main() {
    let key = env::var("PGKEY").expect("PG Api Key");
    let host = env::var("PGHOST").expect("PG Host");

    let clt = client::Client::new(&host, &key).expect("client value");

    let req = toxicity::Request {
        text: "Every flight I have is late and I am very angry. I want to hurt someone."
            .to_string(),
    };

    let result = clt.toxicity(&req).await.expect("error from toxicity");

    println!("\n\ntoxicity response:\n{:?}\n\n", result);
}
