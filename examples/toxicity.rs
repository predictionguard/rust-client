use std::env;

extern crate pg_rust_client as pg_client;
use pg_client::{client, toxicity};

#[tokio::main]
async fn main() {
    let key = env::var("PGKEY").expect("PG Api Key");
    let host = env::var("PGHOST").expect("PG Host");

    let clt = client::Client::new(&host, &key).expect("client value");

    let req = toxicity::Request {
        text: "".to_string(),
    };

    let result = clt.toxicity(&req).await.expect("error from toxicity");

    println!("toxicity response:\n{:?}", result);
}
