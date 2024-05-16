use std::env;

extern crate pg_rust_client as pg_client;
use pg_client::{client, pii};

#[tokio::main]
async fn main() {
    let key = env::var("PGKEY").expect("PG Api Key");
    let host = env::var("PGHOST").expect("PG Host");

    let clt = client::Client::new(&host, &key).expect("client value");

    let req = pii::Request {
        prompt: "George Washington was president of the us".to_string(),
        replace: true,
        replace_method: pii::ReplaceMethod::Random,
    };

    let result = clt.pii(&req).await.expect("error from injection");

    println!("\n\npii response:\n\n{:?}", result);
}
