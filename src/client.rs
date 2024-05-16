use eventsource_client::Client as EventClient;
use eventsource_client::SSE;
use futures::TryStreamExt;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    ClientBuilder, Response, StatusCode,
};
use serde::{Deserialize, Serialize};
use std::{fmt, sync::Arc, time::Duration};

use crate::{completion, factuality, injection, pii, toxicity, translate, Result};

static USER_AGENT: &str = "Prediction Guard Rust Client";

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
    pub fn new(host: &str, api_key: &str) -> Result<Self> {
        // TODO: Allow options to be passed in.
        let http = ClientBuilder::new()
            .connect_timeout(Duration::new(15, 0))
            .read_timeout(Duration::new(30, 0))
            .timeout(Duration::new(45, 0))
            .user_agent(USER_AGENT)
            .build()?;

        let header_key = match HeaderValue::from_str(api_key) {
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
            server: host.to_string(),
            http_client: http,
            headers: header_map,
            api_key: api_key.to_string(),
        });

        Ok(Self { inner })
    }

    /// Calls the generate completion endpoint. It requires an instance of [`completion::Request`]
    /// and returns a [`completion::Response`]. A 200 (Ok) status code is expected from the Prediction Guard api. Any other status code
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

    /// Calls the generate chat completion endpoint. It requires an instance of
    /// [`completion::ChatRequest`] and returns a [`completion::ChatResponse`]. A 200 (Ok) status code is expected from the Prediction Guard api. Any other status code
    /// is considered an error.
    pub async fn generate_chat_completion(
        &self,
        req: &completion::ChatRequest,
    ) -> Result<Option<completion::ChatResponse>> {
        let url = format!("{}{}", &self.inner.server, completion::CHAT_PATH);

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

        let chat_response = result.json::<completion::ChatResponse>().await?;

        Ok(Some(chat_response))
    }

    pub async fn generate_chat_completion_stream<F>(
        &self,
        mut req: completion::ChatRequestEvents,
        callback: &mut F,
    ) -> Result<Option<completion::ChatResponseEvents>>
    where
        F: FnMut(&String),
    {
        //add channel
        let url = format!("{}{}", &self.inner.server, completion::CHAT_PATH);

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
                            let resp: completion::ChatResponseEvents =
                                match serde_json::from_str(&evt.data) {
                                    Ok(v) => v,
                                    Err(e) => {
                                        return Err(Box::from(ApiError {
                                            http_status: 500,
                                            error: format!("error parsing stream response: {}", e),
                                        }))
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
                                        .unwrap_or(completion::ChatEventsDelta {
                                            content: Some("".to_string()),
                                        })
                                        .content;
                                    callback(&msg.unwrap_or("".to_string()));
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

    /// Calls the factuality check endpoint. It requires an instance of [`factuality::Request`]
    /// and returns a [`factuality::Response`]. A 200 (Ok) status code is expected from the Prediction Guard api. Any other status code
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

    /// Calls the translate endpoint. It requires an instance of [`translate::Request`]
    /// and returns a [`translate::Response`]. A 200 (Ok) status code is expected from the Prediction Guard api. Any other status code
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

    /// Calls the PII endpoint that is used to remove/detect PII information in the request. It
    /// requires an instance of [`pii::Request`] and returns [`pii::Response`]. A 200 (Ok) status code is expected from the Prediction Guard api.
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
