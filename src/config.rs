//! Runtime configuration from the environment, resolved once at startup.
//! Tests build a `Config` by hand instead of touching process env vars.

use std::time::Duration;

use crate::featured::{self, FeaturedSource};

/// Everything the api reads from the environment.
#[derive(Debug, Clone)]
pub struct Config {
    /// `APP_NAME`: the shared `name` prop and the health page title.
    pub app_name: String,
    /// `APP_URL` without a trailing slash: where a tab left open on the old
    /// site is sent back to.
    pub app_url: String,
    pub github: GitHubConfig,
    pub esi: EsiConfig,
    /// The hand-picked projects on the landing page (`featured::PROJECTS`).
    pub featured: Vec<FeaturedSource>,
    pub upstream: UpstreamConfig,
}

#[derive(Debug, Clone)]
pub struct GitHubConfig {
    /// `GITHUB_TOKEN`; `None` when unset or blank, which leaves GitHub out
    /// of the page entirely.
    pub token: Option<String>,
    /// `GITHUB_API_URL`, `https://api.github.com` by default (tests and the
    /// end-to-end stack point it at a mock).
    pub api_url: String,
}

#[derive(Debug, Clone)]
pub struct EsiConfig {
    /// `EVE_CHARACTER_ID`, the pilot in the dossier; `None` when unset or
    /// blank.
    pub character_id: Option<String>,
    /// `ESI_URL`, `https://esi.evetech.net` by default.
    pub base_url: String,
}

/// How the GitHub and ESI clients behave on the wire.
#[derive(Debug, Clone)]
pub struct UpstreamConfig {
    /// Waits between attempts: a request is tried once more per entry.
    pub retry_delays: Vec<Duration>,
    pub timeout: Duration,
    pub connect_timeout: Duration,
}

pub const DEFAULT_APP_NAME: &str = "Nicolas Kion";
pub const DEFAULT_APP_URL: &str = "https://nicolaskion.dev";
pub const DEFAULT_GITHUB_API_URL: &str = "https://api.github.com";
pub const DEFAULT_ESI_URL: &str = "https://esi.evetech.net";

impl Default for UpstreamConfig {
    /// Two retries after 250 ms and 1 s, a 10 s timeout and 5 s to
    /// connect. The same as the Laravel HTTP client was configured with.
    fn default() -> Self {
        Self {
            retry_delays: vec![Duration::from_millis(250), Duration::from_secs(1)],
            timeout: Duration::from_secs(10),
            connect_timeout: Duration::from_secs(5),
        }
    }
}

/// A variable that is set and not blank (Laravel's `filled()`), trimmed.
pub fn filled(value: Option<String>) -> Option<String> {
    value
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
}

fn env(key: &str) -> Option<String> {
    filled(std::env::var(key).ok())
}

fn env_or(key: &str, default: &str) -> String {
    env(key).unwrap_or_else(|| default.to_owned())
}

fn without_trailing_slash(url: String) -> String {
    url.trim_end_matches('/').to_owned()
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            app_name: env_or("APP_NAME", DEFAULT_APP_NAME),
            app_url: without_trailing_slash(env_or("APP_URL", DEFAULT_APP_URL)),
            github: GitHubConfig {
                token: env("GITHUB_TOKEN"),
                api_url: without_trailing_slash(env_or("GITHUB_API_URL", DEFAULT_GITHUB_API_URL)),
            },
            esi: EsiConfig {
                character_id: env("EVE_CHARACTER_ID"),
                base_url: without_trailing_slash(env_or("ESI_URL", DEFAULT_ESI_URL)),
            },
            featured: featured::PROJECTS.to_vec(),
            upstream: UpstreamConfig::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::filled;

    #[test]
    fn blank_values_are_not_filled() {
        assert_eq!(filled(None), None);
        assert_eq!(filled(Some(String::new())), None);
        assert_eq!(filled(Some("  \t ".to_owned())), None);
        assert_eq!(filled(Some(" token ".to_owned())), Some("token".to_owned()));
    }
}
