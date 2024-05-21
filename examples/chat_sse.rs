//! `chat_sse` sends a prompt to Prediction Guard and returns a single reponse of
//! type [`completion::ChatResponseEvents`]. The event handler function is called
//! every time a server event is received.
use std::io::Write;

extern crate prediction_guard as pg_client;
use pg_client::{client, completion};

#[tokio::main]
async fn main() {
    let pg_env = client::PgEnvironment::from_env().expect("env keys");

    let clt = client::Client::new(pg_env).expect("client value");

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

    let mut evt_handler = |msg: &String| {
        let _ = buf.write(msg.as_bytes());
        let _ = buf.flush();
    };

    let result = clt
        .generate_chat_completion_events(req, &mut evt_handler)
        .await
        .expect("error from chat_events");

    println!("\n\nchat sse completion response:\n{:?}\n\n", result);
}
