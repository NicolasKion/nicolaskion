//! The HTTP client the GitHub and ESI services share: JSON over reqwest
//! with the configured timeouts and retries.

use reqwest::header::HeaderMap;
use serde::de::DeserializeOwned;

use crate::config::UpstreamConfig;

/// The User-Agent every upstream request carries, so GitHub and CCP can
/// tell who is asking.
pub const USER_AGENT: &str = "nicolaskion.dev (+https://github.com/NicolasKion/nicolaskion)";

/// Why an upstream request gave up, after its retries.
#[derive(Debug)]
pub enum UpstreamError {
    /// The request did not complete (connect, timeout, body).
    Transport(String),
    /// The server answered with a non-success status.
    Status(u16),
    /// The answer was not the JSON shape expected.
    Decode(String),
}

impl std::fmt::Display for UpstreamError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UpstreamError::Transport(error) => write!(f, "request failed: {error}"),
            UpstreamError::Status(status) => write!(f, "answered {status}"),
            UpstreamError::Decode(error) => write!(f, "unexpected answer: {error}"),
        }
    }
}

impl std::error::Error for UpstreamError {}

#[derive(Debug, Clone)]
pub struct Upstream {
    client: reqwest::Client,
    config: UpstreamConfig,
}

impl Upstream {
    pub fn new(config: UpstreamConfig) -> Self {
        let client = reqwest::Client::builder()
            .user_agent(USER_AGENT)
            .timeout(config.timeout)
            .connect_timeout(config.connect_timeout)
            .build()
            .expect("HTTP client builds");
        Self { client, config }
    }

    /// GETs `url` and decodes the JSON answer. Failed requests (no answer,
    /// or a non-success status) are tried again after each retry delay;
    /// an answer that does not decode is not.
    pub async fn get_json<T: DeserializeOwned>(
        &self,
        url: &str,
        query: &[(&str, &str)],
        headers: HeaderMap,
    ) -> Result<T, UpstreamError> {
        let mut delays = self.config.retry_delays.iter();
        loop {
            let error = match self.attempt(url, query, headers.clone()).await {
                Ok(response) => {
                    let body = response
                        .bytes()
                        .await
                        .map_err(|error| UpstreamError::Transport(error.to_string()));
                    match body {
                        Ok(body) => {
                            return serde_json::from_slice(&body)
                                .map_err(|error| UpstreamError::Decode(error.to_string()));
                        }
                        Err(error) => error,
                    }
                }
                Err(error) => error,
            };
            match delays.next() {
                Some(delay) => tokio::time::sleep(*delay).await,
                None => return Err(error),
            }
        }
    }

    /// GETs `url` and returns the status and the raw body of the last
    /// attempt; failed answers are retried like `get_json`, and only a
    /// request that never got an answer is an error.
    pub async fn get_bytes(&self, url: &str) -> Result<(u16, Vec<u8>), UpstreamError> {
        let mut delays = self.config.retry_delays.iter();
        loop {
            let outcome = match self.client.get(url).send().await {
                Ok(response) => {
                    let status = response.status().as_u16();
                    match response.bytes().await {
                        Ok(body) => Ok((status, body.to_vec())),
                        Err(error) => Err(UpstreamError::Transport(error.to_string())),
                    }
                }
                Err(error) => Err(UpstreamError::Transport(error.to_string())),
            };
            let failed = !matches!(&outcome, Ok((status, _)) if (200..300).contains(status));
            match delays.next() {
                Some(delay) if failed => tokio::time::sleep(*delay).await,
                _ => return outcome,
            }
        }
    }

    /// One request; a non-success status is an error.
    async fn attempt(
        &self,
        url: &str,
        query: &[(&str, &str)],
        headers: HeaderMap,
    ) -> Result<reqwest::Response, UpstreamError> {
        let response = self
            .client
            .get(url)
            .query(query)
            .headers(headers)
            .send()
            .await
            .map_err(|error| UpstreamError::Transport(error.to_string()))?;
        if response.status().is_success() {
            Ok(response)
        } else {
            Err(UpstreamError::Status(response.status().as_u16()))
        }
    }
}
