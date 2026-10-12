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

    /// GETs `url` and decodes the JSON answer.
    pub async fn get_json<T: DeserializeOwned>(
        &self,
        _url: &str,
        _query: &[(&str, &str)],
        _headers: HeaderMap,
    ) -> Result<T, UpstreamError> {
        let _ = (&self.client, &self.config);
        Err(UpstreamError::Transport("not implemented".to_owned()))
    }

    /// GETs `url` and returns the status and the raw body.
    pub async fn get_bytes(&self, _url: &str) -> Result<(u16, Vec<u8>), UpstreamError> {
        Err(UpstreamError::Transport("not implemented".to_owned()))
    }
}
