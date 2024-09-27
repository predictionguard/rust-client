//! `chat_sse` sends a prompt to Prediction Guard and returns a single response of
//! type [`chat::Response`]. The event handler function is called
//! every time a server event is received.
extern crate prediction_guard as pg_client;

use std::io::Write;

use pg_client::{chat, client};

#[tokio::main]
async fn main() {
    let pg_env = client::PgEnvironment::from_env().expect("env keys");

    let clt = client::Client::new(pg_env).expect("client value");

    // Load the list of models available for chat completion.
    let models = clt
        .retrieve_chat_completion_models()
        .await
        .expect("model list");

    assert!(!models.is_empty());

    let mut req = chat::Request::<chat::Message>::new(models[0].clone())
        .add_message(
            chat::Roles::User,
            "How do you feel about the world in general".to_string(),
        )
        .max_tokens(300)
        .temperature(0.1)
        .top_p(0.1)
        .top_k(50);

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
