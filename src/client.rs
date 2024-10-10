//! Used to connect to the Prediction Guard API.
use std::{env, fmt, sync::Arc, time::Duration};

use crate::built_info;
use crate::{chat, completion, embedding, factuality, injection, pii, toxicity, translate, Result};
use dotenvy;
use eventsource_client::Client as EventClient;
use eventsource_client::SSE;
use futures::TryStreamExt;
use log::error;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    ClientBuilder, Response, StatusCode,
};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::Sender;

const USER_AGENT: &str = "Prediction Guard Rust Client";

/// The base error that is returned from the API calls.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ApiError {
    error: String,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("error {}", self.error))
    }
}

impl std::error::Error for ApiError {}

/// Prediction Guard Configuration
pub struct PgEnvironment {
    pub key: String,
    pub host: String,
}

impl PgEnvironment {
    /// Specify Prediction Guard API configuration manually
    ///
    /// ## Arguments:
    ///
    /// * `key` - the Prediction Guard API key
    /// * `host` - the Prediction Guard URL
    pub fn new(key: String, host: String) -> Self {
        Self { key, host }
    }

    /// Loads Prediction Guard API configuration from either
    /// a `.env` file or environment variables. Expects to find
    /// the `PREDICTIONGUARD_API_KEY` and `PREDICTIONGUARD_URL` environment variables.
    ///
    /// Returns an error if the environment variables are not found.
    pub fn from_env() -> Result<Self> {
        let _ = dotenvy::dotenv(); // Ignoring error - it's ok to not have .env files
        Ok(Self {
            key: env::var("PREDICTIONGUARD_API_KEY")?,
            host: env::var("PREDICTIONGUARD_URL")?,
        })
    }
}

/// Handles the connectivity to the Prediction Guard API. It is safe to be
/// used across threads.
#[derive(Debug, Clone)]
pub struct Client {
    inner: Arc<ClientInner>,
}

#[derive(Debug)]
struct ClientInner {
    server: String,
    http_client: reqwest::Client,
    headers: HeaderMap,
    api_key: String,
}

impl Client {
    /// Creates a new instance of client to be used. Assumes the Prediction Guard keys,
    /// PREDICTIONGUARD_API_KEY and PREDICTIONGUARD_URL are set in the environment.
    pub fn new() -> Result<Self> {
        let pg_env = PgEnvironment::from_env().expect("env keys");

        Self::from_environment(pg_env)
    }

    /// Creates a new instance of client to be used with a particular Prediction Guard environment.
    ///
    ///  ## Arguments:
    ///
    ///  * `pg_env` - the prediction guard environment to connect to.
    pub fn from_environment(pg_env: PgEnvironment) -> Result<Self> {
        let user_agent = format!("{} v{}", USER_AGENT, built_info::PKG_VERSION);

        let http = ClientBuilder::new()
            .connect_timeout(Duration::new(30, 0))
            .read_timeout(Duration::new(30, 0))
            .timeout(Duration::new(45, 0))
            .user_agent(user_agent)
            .build()?;

        let header_key = match HeaderValue::from_str(&pg_env.key) {
            Ok(x) => x,
            Err(e) => {
                return Err(Box::new(e));
            }
        };

        let mut header_map = HeaderMap::new();
        let _ = header_map
            .insert("x-api-key", header_key)
            .ok_or("invalid api key");

        let inner = Arc::new(ClientInner {
            server: pg_env.host.to_string(),
            http_client: http,
            headers: header_map,
            api_key: pg_env.key,
        });

        Ok(Self { inner })
    }

    /// Calls the health endpoint.
    ///
    /// Returns the text response from the server. A 200 (Ok) status code is expected from
    /// Prediction Guard api. Any other status code is considered an error.
    pub async fn check_health(&self) -> Result<Option<String>> {
        let result = self
            .inner
            .http_client
            .get(&self.inner.server)
            .headers(self.inner.headers.clone())
            .send()
            .await?;

        if result.status() != StatusCode::OK {
            return Err(retrieve_error(result).await);
        }

        let txt = result.text().await?;

        Ok(Some(txt))
    }

    /// Calls the embedding endpoint.
    ///
    /// ## Arguments:
    ///
    /// * `req` - An instance of [`embedding::Request`]
    ///
    /// Returns a [`embedding::Response`]. A 200 (Ok) status code is expected from the Prediction Guard api. Any other status code
    /// is considered an error.
    pub async fn embedding(&self, req: &embedding::Request) -> Result<Option<embedding::Response>> {
        let url = format!("{}{}", &self.inner.server, embedding::PATH);

        let result = self
            .inner
            .http_client
            .post(url)
            .headers(self.inner.headers.clone())
            .json(req)
            .send()
            .await?;

        if result.status() != StatusCode::OK {
            return Err(retrieve_error(result).await);
        }

        let embed_response = result.json::<embedding::Response>().await?;

        Ok(Some(embed_response))
    }

    /// Retrieves the list of models available for the embeddings endpoint.
    ///
    /// Returns a vector of strings with the model names. A 200 (Ok) status code is expected from the Prediction Guard api. Any other status code
    /// is considered an error.
    pub async fn retrieve_embedding_models(&self) -> Result<Vec<String>> {
        let url = format!("{}{}", &self.inner.server, embedding::PATH);

        let result = self
            .inner
            .http_client
            .get(url)
            .headers(self.inner.headers.clone())
            .send()
            .await?;

        if result.status() != StatusCode::OK {
            return Err(retrieve_error(result).await);
        }

        let embed_response = result.json::<Vec<String>>().await?;

        Ok(embed_response)
    }

    /// Calls the generate completion endpoint.
    ///
    /// ## Arguments:
    ///
    /// * `req` - An instance of [`completion::Request`]
    ///
    /// Returns a [`completion::Response`]. A 200 (Ok) status code is expected from the Prediction Guard api. Any other status code
    /// is considered an error.
    pub async fn generate_completion(
        &self,
        req: &completion::Request,
    ) -> Result<Option<completion::Response>> {
        let url = format!("{}{}", &self.inner.server, completion::PATH);

        let result = self
            .inner
            .http_client
            .post(url)
            .headers(self.inner.headers.clone())
            .json(req)
            .send()
            .await?;

        if result.status() != StatusCode::OK {
            return Err(retrieve_error(result).await);
        }

        let comp_response = result.json::<completion::Response>().await?;

        Ok(Some(comp_response))
    }

    /// Retrieves the list of models available for the completion endpoint.
    ///
    /// Returns a vector of strings with the model names. A 200 (Ok) status code is expected from the Prediction Guard api. Any other status code
    /// is considered an error.
    pub async fn retrieve_completion_models(&self) -> Result<Vec<String>> {
        let url = format!("{}{}", &self.inner.server, completion::PATH);

        let result = self
            .inner
            .http_client
            .get(url)
            .headers(self.inner.headers.clone())
            .send()
            .await?;

        if result.status() != StatusCode::OK {
            return Err(retrieve_error(result).await);
        }

        let comp_response = result.json::<Vec<String>>().await?;

        Ok(comp_response)
    }

    /// Calls the generate chat completion endpoint.
    ///
    /// ## Arguments:
    ///
    /// * `req` - An instance of [`chat::Request::<Message>`]
    ///
    /// Returns an instance of [`chat::Response`]. A 200 (Ok) status code is expected from the Prediction Guard api. Any other status code
    /// is considered an error.
    pub async fn generate_chat_completion(
        &self,
        req: &chat::Request<chat::Message>,
    ) -> Result<Option<chat::Response>> {
        let url = format!("{}{}", &self.inner.server, chat::PATH);

        let result = self
            .inner
            .http_client
            .post(url)
            .headers(self.inner.headers.clone())
            .json(req)
            .send()
            .await?;

        if result.status() != StatusCode::OK {
            return Err(retrieve_error(result).await);
        }

        let chat_response = result.json::<chat::Response>().await?;

        Ok(Some(chat_response))
    }

    /// Calls the generate chat completion endpoint.
    ///
    /// ## Arguments:
    ///
    /// * `req` - An instance of [`chat::Request::<Message>`]
    /// * `event_handler` - Event handler function that is called when a server side event is raised.
    ///
    /// Returns an instance of [`chat::Response`].
    ///
    /// The generated text is returned via events from the server. The event handler function gets called
    /// every time the client receives an event response with data. Once the server terminates the events the call returns.
    /// The entire [`chat::Response`] response is then returned to the caller.
    ///
    /// A 200 (Ok) status code is expected from the Prediction Guard api. Any other status code
    /// is considered an error.
    pub async fn generate_chat_completion_events<F>(
        &self,
        req: &mut chat::Request<chat::Message>,
        event_handler: &mut F,
    ) -> Result<Option<chat::ResponseEvents>>
    where
        F: FnMut(&String),
    {
        let url = format!("{}{}", &self.inner.server, chat::PATH);

        req.stream = true;
        req.output = None;

        let body = serde_json::to_string(&req)?;

        let user_agent = format!("{} v{}", USER_AGENT, built_info::PKG_VERSION);

        let key = format!("Bearer {}", &self.inner.api_key);

        let client = eventsource_client::ClientBuilder::for_url(&url)?
            .header("User-Agent", user_agent.as_str())?
            .header("Authorization", &key)?
            .method("POST".to_string())
            .body(body)
            .build();

        let mut stream = Box::pin(client.stream());

        loop {
            match stream.try_next().await {
                Ok(Some(event)) => {
                    match event {
                        SSE::Comment(_) => continue,
                        SSE::Event(evt) => {
                            // Check for [DONE]
                            if evt.data == "[DONE]" {
                                return Ok(None);
                            }

                            // JSON Response
                            let resp: chat::ResponseEvents = match serde_json::from_str(&evt.data) {
                                Ok(v) => v,
                                Err(e) => {
                                    return Err(Box::from(ApiError {
                                        error: format!("error parsing stream response: {}", e),
                                    }));
                                }
                            };

                            if resp.choices.is_empty() {
                                // No data to stream or Done
                                continue;
                            }

                            // Finish Reason == Stop That is the final Response.
                            if resp.choices[0].finish_reason == Some("stop".to_string()) {
                                return Ok(Some(resp));
                            }

                            let msg = resp.choices[0].delta.clone().content;
                            event_handler(&msg);
                        }
                    }
                }

                Ok(None) => continue,
                Err(e) => match e {
                    eventsource_client::Error::StreamClosed => break,
                    _ => return Err(stream_error_into_api_err(e).await),
                },
            }
        }

        Ok(None)
    }

    /// Calls the generate chat completion endpoint.
    ///
    /// ## Arguments:
    ///
    /// * `req` - An instance of [`chat::Request::<Message>`]
    /// * `sender` - A sender instance for a channel where there is a receiver waiting for a message.
    ///
    /// Returns an instance of [`chat::Response`].
    ///
    /// The generated text is returned via events from the server. The sender gets called
    /// every time the client receives an event response with data. Once the server terminates the events the call returns.
    /// The receiver should handle the `STOP` message which means there are no more messages to receive and exit.
    /// The entire [`chat::Response`] response is then returned to the caller.
    ///
    /// A 200 (Ok) status code is expected from the Prediction Guard api. Any other status code
    /// is considered an error.
    pub async fn generate_chat_completion_events_async(
        &self,
        req: &mut chat::Request<chat::Message>,
        sender: &Sender<String>,
    ) -> Result<Option<chat::ResponseEvents>> {
        let url = format!("{}{}", &self.inner.server, chat::PATH);

        req.stream = true;
        req.output = None;

        let body = serde_json::to_string(&req)?;

        let user_agent = format!("{} v{}", USER_AGENT, built_info::PKG_VERSION);

        let key = format!("Bearer {}", &self.inner.api_key);

        let client = eventsource_client::ClientBuilder::for_url(&url)?
            .header("User-Agent", user_agent.as_str())?
            .header("Authorization", &key)?
            .method("POST".to_string())
            .body(body)
            .build();

        let mut stream = Box::pin(client.stream());

        loop {
            match stream.try_next().await {
                Ok(Some(event)) => {
                    match event {
                        SSE::Comment(_) => continue,
                        SSE::Event(evt) => {
                            // Check for [DONE]
                            if evt.data == "[DONE]" {
                                let _ = sender.send("STOP".to_string()).await;
                                return Ok(None);
                            }

                            // JSON Response
                            let resp: chat::ResponseEvents = match serde_json::from_str(&evt.data) {
                                Ok(v) => v,
                                Err(e) => {
                                    return Err(Box::from(ApiError {
                                        error: format!("error parsing stream response: {}", e),
                                    }));
                                }
                            };

                            if resp.choices.is_empty() {
                                // No data to stream or Done
                                continue;
                            }

                            // Finish Reason == Stop That is the final Response.
                            if resp.choices[0].finish_reason == Some("stop".to_string()) {
                                let _ = sender.send("STOP".to_string()).await;
                                return Ok(Some(resp));
                            }

                            let msg = resp.choices[0].delta.clone().content;

                            match sender.send(msg).await {
                                Ok(_) => (),
                                Err(e) => {
                                    error!("generate_chat_completion_events_async - error sending on channel, {e}");
                                }
                            }
                        }
                    }
                }

                Ok(None) => continue,
                Err(e) => match e {
                    eventsource_client::Error::StreamClosed => break,
                    _ => return Err(stream_error_into_api_err(e).await),
                },
            }
        }

        Ok(None)
    }

    /// Retrieves the list of models available for the chat completion endpoint.
    ///
    /// Returns a vector of strings with the model names. A 200 (Ok) status code is expected from the Prediction Guard api. Any other status code
    /// is considered an error.
    pub async fn retrieve_chat_completion_models(&self) -> Result<Vec<String>> {
        let url = format!("{}{}", &self.inner.server, chat::PATH);

        let result = self
            .inner
            .http_client
            .get(url)
            .headers(self.inner.headers.clone())
            .send()
            .await?;

        if result.status() != StatusCode::OK {
            return Err(retrieve_error(result).await);
        }

        let chat_response = result.json::<Vec<String>>().await?;

        Ok(chat_response)
    }

    /// Calls the generate chat completion endpoint for chat vision.
    ///
    /// ## Arguments:
    ///
    /// * `req` - An instance of [`chat::Request::<MessageVision>`]
    ///
    /// Returns an instance of [`chat::Response`]. A 200 (Ok) status code is expected from the Prediction Guard api. Any other status code
    /// is considered an error.
    pub async fn generate_chat_vision(
        &self,
        req: &chat::Request<chat::MessageVision>,
    ) -> Result<Option<chat::Response>> {
        let url = format!("{}{}", &self.inner.server, chat::PATH);

        let result = self
            .inner
            .http_client
            .post(url)
            .headers(self.inner.headers.clone())
            .json(req)
            .send()
            .await?;

        if result.status() != StatusCode::OK {
            return Err(retrieve_error(result).await);
        }

        let chat_response = result.json::<chat::Response>().await?;

        Ok(Some(chat_response))
    }

    /// Retrieves the list of models available for the chat vision endpoint.
    ///
    /// Returns a vector of strings with the model names. A 200 (Ok) status code is expected from the Prediction Guard api. Any other status code
    /// is considered an error.
    pub async fn retrieve_chat_vision_models(&self) -> Result<Vec<String>> {
        let url = format!("{}{}", &self.inner.server, chat::PATH_VISION_MODELS);

        let result = self
            .inner
            .http_client
            .get(url)
            .headers(self.inner.headers.clone())
            .send()
            .await?;

        if result.status() != StatusCode::OK {
            return Err(retrieve_error(result).await);
        }

        let chat_vision = result.json::<Vec<String>>().await?;

        Ok(chat_vision)
    }

    /// Calls the factuality check endpoint.
    ///
    /// ## Arguments:
    ///
    /// * `req` - An instance of [`factuality::Request`]
    ///
    /// Returns am instance of [`factuality::Response`]. A 200 (Ok) status code is expected from the Prediction Guard api. Any other status code
    /// is considered an error.
    pub async fn check_factuality(
        &self,
        req: &factuality::Request,
    ) -> Result<Option<factuality::Response>> {
        let url = format!("{}{}", &self.inner.server, factuality::PATH);

        let result = self
            .inner
            .http_client
            .post(url)
            .headers(self.inner.headers.clone())
            .json(req)
            .send()
            .await?;

        if result.status() != StatusCode::OK {
            return Err(retrieve_error(result).await);
        }

        let fact_response = result.json::<factuality::Response>().await?;

        Ok(Some(fact_response))
    }

    /// Calls the translate endpoint.
    ///
    /// ## Arguments:
    ///
    /// `req` - Instance of [`translate::Request`]
    ///
    /// Returns a [`translate::Response`]. A 200 (Ok) status code is expected from the Prediction Guard api. Any other status code
    /// is considered an error.
    pub async fn translate(&self, req: &translate::Request) -> Result<Option<translate::Response>> {
        let url = format!("{}{}", &self.inner.server, translate::PATH);

        let result = self
            .inner
            .http_client
            .post(url)
            .headers(self.inner.headers.clone())
            .json(req)
            .send()
            .await?;

        if result.status() != StatusCode::OK {
            return Err(retrieve_error(result).await);
        }

        let translate_response = result.json::<translate::Response>().await?;

        Ok(Some(translate_response))
    }

    /// Calls the PII endpoint that is used to remove/detect PII information in the request.
    ///
    /// ## Arguments:
    ///
    /// `req` - An instance of [`pii::Request`]
    ///
    /// Returns an instance of [`pii::Response`]. A 200 (Ok) status code is expected from the Prediction Guard api.
    /// Any other status code is considered an error.
    pub async fn pii(&self, req: &pii::Request) -> Result<Option<pii::Response>> {
        let url = format!("{}{}", &self.inner.server, pii::PATH);

        let result = self
            .inner
            .http_client
            .post(url)
            .headers(self.inner.headers.clone())
            .json(req)
            .send()
            .await?;

        if result.status() != StatusCode::OK {
            return Err(retrieve_error(result).await);
        }

        let pii_response = result.json::<pii::Response>().await?;

        Ok(Some(pii_response))
    }

    /// Calls the injection check endpoint.
    ///
    /// ## Arguments:
    ///
    /// `req` - Instance of [`injection::Request`]
    ///
    /// Returns an instance of [`injection::Response`]. A 200 (Ok) status code is expected from the Prediction Guard api. Any other status code
    /// is considered an error.
    pub async fn injection(&self, req: &injection::Request) -> Result<Option<injection::Response>> {
        let url = format!("{}{}", &self.inner.server, injection::PATH);

        let result = self
            .inner
            .http_client
            .post(url)
            .headers(self.inner.headers.clone())
            .json(req)
            .send()
            .await?;

        if result.status() != StatusCode::OK {
            return Err(retrieve_error(result).await);
        }

        let injection_response = result.json::<injection::Response>().await?;

        Ok(Some(injection_response))
    }

    /// Calls the injection check endpoint.
    ///
    /// ## Arguments:
    ///
    /// `req` - An instance of [`toxicity::Request`]
    ///
    /// Returns an instance of [`toxicity::Response`]. A 200 (Ok) status code is expected from the Prediction Guard api. Any other status code
    /// is considered an error.
    pub async fn toxicity(&self, req: &toxicity::Request) -> Result<Option<toxicity::Response>> {
        let url = format!("{}{}", &self.inner.server, toxicity::PATH);

        let result = self
            .inner
            .http_client
            .post(url)
            .headers(self.inner.headers.clone())
            .json(req)
            .send()
            .await?;

        if result.status() != StatusCode::OK {
            return Err(retrieve_error(result).await);
        }

        let toxicity_response = result.json::<toxicity::Response>().await?;

        Ok(Some(toxicity_response))
    }
}

async fn retrieve_error(resp: Response) -> Box<dyn std::error::Error> {
    let err = match resp.json::<ApiError>().await {
        Ok(x) => x,
        Err(e) => return Box::from(format!("error parsing error response, {}", e)),
    };

    err.into()
}

async fn stream_error_into_api_err(err: eventsource_client::Error) -> Box<dyn std::error::Error> {
    let msg = format!("{}", err);
    Box::from(ApiError {
        error: msg.to_string(),
    })
}
