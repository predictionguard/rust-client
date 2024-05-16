use std::env;
use std::io::Write;

extern crate pg_rust_client as pg_client;
use pg_client::{client, completion};

#[tokio::main]
async fn main() {
    let key = env::var("PGKEY").expect("PG Api Key");
    let host = env::var("PGHOST").expect("PG Host");

    let clt = client::Client::new(&host, &key).expect("client value");

    let req = completion::ChatRequestEvents {
        model: completion::Models::NeuralChat7B,
        messages: vec![completion::Message {
            role: completion::Roles::User,
            content: "How do you feel about the world in general".to_string(),
        }],
        max_tokens: 1000,
        temperature: 1.1,
        stream: true,
    };

    let lock = std::io::stdout().lock();
    let mut buf = std::io::BufWriter::new(lock);

    let mut callback = |msg: &String| {
        let _ = buf.write(msg.as_bytes());
        let _ = buf.flush();
    };

    let result = clt
        .generate_chat_completion_stream(req, &mut callback)
        .await
        .expect("error from generate chat completion");

    println!("\n\nchat sse completion response:\n{:?}", result);
}
