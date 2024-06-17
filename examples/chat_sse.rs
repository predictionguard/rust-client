//! `chat_sse` sends a prompt to Prediction Guard and returns a single response of
//! type [`chat::Response`]. The event handler function is called
//! every time a server event is received.
extern crate prediction_guard as pg_client;

use std::io::Write;

use pg_client::{chat, client, models};

#[tokio::main]
async fn main() {
    let pg_env = client::PgEnvironment::from_env().expect("env keys");

    let clt = client::Client::new(pg_env).expect("client value");

    let mut req = chat::Request::<chat::Message>::new(models::Model::NeuralChat7B)
        .add_message(
            chat::Roles::User,
            "How do you feel about the world in general".to_string(),
        )
        .max_tokens(1000)
        .temperature(0.85);

    let lock = std::io::stdout().lock();
    let mut buf = std::io::BufWriter::new(lock);

    let mut evt_handler = |msg: &String| {
        let _ = buf.write(msg.as_bytes());
        let _ = buf.flush();
    };

    let result = clt
        .generate_chat_completion_events(&mut req, &mut evt_handler)
        .await
        .expect("error from chat_events");

    println!("\n\nchat sse completion response:\n{:?}\n\n", result);
}
