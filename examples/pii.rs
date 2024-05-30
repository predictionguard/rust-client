//! `pii` sends a prompt to Prediction Guard and returns a single response of
//! type [`pii::Response`].
extern crate prediction_guard as pg_client;

use pg_client::{client, pii};
use pii::ReplaceMethod;

#[tokio::main]
async fn main() {
    let pg_env = client::PgEnvironment::from_env().expect("env keys");

    let clt = client::Client::new(pg_env).expect("client value");

    let req = pii::Request {
        prompt: "My email is joe@gmail.com and my number is 270-123-4567".to_string(),
        replace: true,
        replace_method: ReplaceMethod::Mask,
    };

    let result = clt.pii(&req).await.expect("error from pii");

    println!("\n\npii response:\n{:?}\n\n", result);
}
