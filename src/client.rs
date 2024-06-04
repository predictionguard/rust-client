//! Used to connect to the Prediction Guard API.
use std::{env, fmt, sync::Arc, time::Duration};

use dotenvy;
use eventsource_client::Client as EventClient;
use eventsource_client::SSE;
use futures::TryStreamExt;
use reqwest::{
    ClientBuilder,
    header::{HeaderMap, HeaderValue}, Response, StatusCode,
};
use serde::{Deserialize, Serialize};

use crate::{chat, completion, embedding, factuality, injection, pii, Result, toxicity, translate};

const USER_AGENT: &str = "Prediction Guard Rust Client";

/// The base error that is returned from the API calls.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ApiError {
    #[serde(default)]
    http_status: u16,
    error: String,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "http_status {}, error {}",
            self.http_status, self.error
        ))
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
    /// the `PGKEY` and `PGHOST` environment variables.
    ///
    /// Returns an error if the environment variables are not found.
    pub fn from_env() -> Result<Self> {
        let _ = dotenvy::dotenv(); // Ignoring error - it's ok to not have .env files
        Ok(Self {
            key: env::var("PGKEY")?,
            host: env::var("PGHOST")?,
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
    /// Creates a new instance of client to be used with a particular Prediction Guard environment.
    ///
    ///  ## Arguments:
    ///
    ///  * `pg_env` - the prediction guard environment to connect to.
    pub fn new(pg_env: PgEnvironment) -> Result<Self> {
        let http = ClientBuilder::new()
            .connect_timeout(Duration::new(15, 0))
            .read_timeout(Duration::new(30, 0))
            .timeout(Duration::new(45, 0))
            .user_agent(USER_AGENT)
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
        mut req: chat::Request<chat::Message>,
        event_handler: &mut F,
    ) -> Result<Option<chat::ResponseEvents>>
    where
        F: FnMut(&String),
    {
        let url = format!("{}{}", &self.inner.server, chat::PATH);

        req.stream = true;

        let body = serde_json::to_string(&req)?;

        let client = eventsource_client::ClientBuilder::for_url(&url)?
            .header("user-agent", USER_AGENT)?
            .header("x-api-key", &self.inner.api_key)?
            .method("POST".to_string())
            .body(body)
            .build();

        // TODO: Add Timeouts
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
                                        http_status: 500,
                                        error: format!("error parsing stream response: {}", e),
                                    }));
                                }
                            };

                            match resp.choices {
                                Some(ref choices) => {
                                    if choices.is_empty() {
                                        // No data to stream or Done
                                        continue;
                                    }

                                    // Finish Reason == Stop That is the final Response.
                                    if choices[0].finish_reason == Some("stop".to_string()) {
                                        return Ok(Some(resp));
                                    }

                                    let msg = choices[0]
                                        .delta
                                        .clone()
                                        .unwrap_or(chat::EventsDelta {
                                            content: Some("".to_string()),
                                        })
                                        .content;
                                    event_handler(&msg.unwrap_or("".to_string()));
                                }
                                None => return Ok(None),
                            }
                        }
                    }
                }
                Ok(None) => continue,
                Err(e) => match e {
                    eventsource_client::Error::StreamClosed => break,
                    _ => return Err(stream_error_into_api_err(e).await),
                },
            };
        }

        Ok(None)
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
    let code = resp.status().as_u16();

    let mut err = match resp.json::<ApiError>().await {
        Ok(x) => x,
        Err(e) => return Box::from(format!("error parsing error response, {}", e)),
    };

    err.http_status = code;
    err.into()
}

async fn stream_error_into_api_err(err: eventsource_client::Error) -> Box<dyn std::error::Error> {
    let msg = format!("{}", err);
    Box::from(ApiError {
        http_status: 500,
        error: msg.to_string(),
    })
}
