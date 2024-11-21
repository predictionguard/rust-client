//! `chat_sse_async` sends a prompt to Prediction Guard and returns a single response of
//! type [`chat::Response`]. It uses a channel to send events back to the reciever
//! allowing for asynchronous processing of the event.
extern crate prediction_guard as pg_client;

use pg_client::{chat, client};
use std::io::Write;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let clt = client::Client::new().expect("client value");

    // Load the list of models available for chat completion.
    let models = clt
        .retrieve_model_list("chat-completion".to_string())
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

    let (tx, mut rx) = mpsc::channel::<String>(32);

    // Launch in separate thread.
    tokio::spawn(async move {
        let result = clt
            .generate_chat_completion_events_async(&mut req, &tx)
            .await
            .expect("error from chat_events");

        println!("\n\nchat sse async completion response:\n{:?}\n\n", result);
    });

    let lock = std::io::stdout().lock();
    let mut buf = std::io::BufWriter::new(lock);

    loop {
        match rx.recv().await {
            Some(msg) => {
                if msg == "STOP".to_string() {
                    break;
                }

                let _ = buf.write(msg.as_bytes());
                let _ = buf.flush();
            }
            None => {
                break;
            }
        }
    }
}
