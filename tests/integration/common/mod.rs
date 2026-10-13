//! Shared harness for the integration suites: the real router with its
//! GitHub and ESI clients pointed at a mock upstream on an ephemeral port,
//! a clock the tests move, and request helpers.

// Not every suite uses every helper.
#![allow(dead_code)]

pub mod upstream;

use std::path::{Path, PathBuf};
use std::time::Duration;

use axum::Router;
use axum::body::Body;
use axum::http::{HeaderMap, Method, Request, StatusCode};
use http_body_util::BodyExt;
use serde_json::Value;
use tower::ServiceExt;

use nicolaskion::cache::Clock;
use nicolaskion::config::{Config, EsiConfig, GitHubConfig, UpstreamConfig};
use nicolaskion::featured::{self, FeaturedSource};
use nicolaskion::server::AppState;

pub use upstream::{MockUpstream, Service};

/// The token and character the tests use.
pub const TOKEN: &str = "fake-token";
pub const CHARACTER_ID: &str = "95569887";

pub fn repo_path(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(relative)
}

/// A config with GitHub and ESI configured against `upstream`, the real
/// featured projects and quick retries (two, like production, 1 ms apart).
pub fn test_config(upstream: &MockUpstream) -> Config {
    Config {
        app_name: "Nicolas Kion".to_owned(),
        github: GitHubConfig {
            token: Some(TOKEN.to_owned()),
            api_url: upstream.github_url(),
        },
        esi: EsiConfig {
            character_id: Some(CHARACTER_ID.to_owned()),
            base_url: upstream.esi_url(),
        },
        featured: featured::PROJECTS.to_vec(),
        upstream: UpstreamConfig {
            retry_delays: vec![Duration::from_millis(1), Duration::from_millis(1)],
            timeout: Duration::from_secs(5),
            connect_timeout: Duration::from_secs(2),
        },
    }
}

/// Two featured projects: one without a repository, one with.
pub fn test_featured() -> Vec<FeaturedSource> {
    vec![
        FeaturedSource {
            logo: Some("/logos/mutamarket.svg"),
            site_url: Some("https://mutamarket.com"),
            ..FeaturedSource::new("MutaMarket", "Closed source marketplace.")
        },
        FeaturedSource {
            logo: Some("/logos/killfeed.svg"),
            repository: Some("WormholeSystems/killfeed"),
            ..FeaturedSource::new("Killfeed", "Open source killmail feed.")
        },
    ]
}

/// One app under test: its router, the mock upstream it talks to and the
/// clock its caches read.
pub struct TestApp {
    pub router: Router,
    pub state: AppState,
    pub upstream: MockUpstream,
    pub clock: Clock,
    pub config: Config,
}

impl TestApp {
    /// GitHub and ESI configured, serving the fixture data.
    pub async fn new() -> Self {
        Self::with(|_| {}).await
    }

    /// Like `new`, with a changed configuration.
    pub async fn with(change: impl FnOnce(&mut Config)) -> Self {
        let upstream = MockUpstream::start().await;
        let mut config = test_config(&upstream);
        change(&mut config);
        let clock = Clock::default();
        let state = AppState::with_clock(config.clone(), clock.clone());
        Self {
            router: nicolaskion::server::router(state.clone()),
            state,
            upstream,
            clock,
            config,
        }
    }

    pub async fn get(&self, path: &str) -> TestResponse {
        self.request(Method::GET, path, &[]).await
    }

    pub async fn get_with(&self, path: &str, headers: &[(&str, &str)]) -> TestResponse {
        self.request(Method::GET, path, headers).await
    }

    /// The landing page props.
    pub async fn home(&self) -> Value {
        let response = self.get("/api/pages/home").await;
        assert_eq!(response.status, StatusCode::OK, "{}", response.text());
        response.json()
    }

    pub async fn request(
        &self,
        method: Method,
        path: &str,
        headers: &[(&str, &str)],
    ) -> TestResponse {
        let mut request = Request::builder().method(method).uri(path);
        for (name, value) in headers {
            request = request.header(*name, *value);
        }
        let response = self
            .router
            .clone()
            .oneshot(request.body(Body::empty()).expect("request builds"))
            .await
            .expect("router answers");
        let status = response.status();
        let headers = response.headers().clone();
        let body = response
            .into_body()
            .collect()
            .await
            .expect("body reads")
            .to_bytes()
            .to_vec();
        TestResponse {
            status,
            headers,
            body,
        }
    }

    /// Waits until the mock has seen `count` requests to `service` (a
    /// background refresh), failing after two seconds.
    pub async fn wait_for_requests(&self, service: Service, count: usize) {
        for _ in 0..200 {
            if self.upstream.requests_to(service).len() >= count {
                // Let the refresh store its result.
                tokio::time::sleep(Duration::from_millis(20)).await;
                return;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
        panic!(
            "expected {count} requests to {service:?}, saw {:?}",
            self.upstream.requests_to(service)
        );
    }
}

pub struct TestResponse {
    pub status: StatusCode,
    pub headers: HeaderMap,
    pub body: Vec<u8>,
}

impl TestResponse {
    pub fn json(&self) -> Value {
        serde_json::from_slice(&self.body).unwrap_or_else(|error| {
            panic!(
                "{} is not JSON ({error}): {}",
                self.status,
                String::from_utf8_lossy(&self.body)
            )
        })
    }

    pub fn text(&self) -> String {
        String::from_utf8_lossy(&self.body).into_owned()
    }

    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers.get(name).and_then(|value| value.to_str().ok())
    }
}

/// Asserts two JSON values are equal with the same key set at every
/// level, naming the first path that differs.
#[track_caller]
pub fn assert_json_eq(expected: &Value, actual: &Value) {
    if let Some(difference) = json_difference("$", expected, actual) {
        panic!("{difference}");
    }
}

pub fn json_difference(path: &str, expected: &Value, actual: &Value) -> Option<String> {
    match (expected, actual) {
        (Value::Object(expected), Value::Object(actual)) => {
            let mut expected_keys: Vec<_> = expected.keys().collect();
            let mut actual_keys: Vec<_> = actual.keys().collect();
            expected_keys.sort();
            actual_keys.sort();
            if expected_keys != actual_keys {
                return Some(format!(
                    "{path}: key sets differ\n  expected {expected_keys:?}\n  actual   {actual_keys:?}"
                ));
            }
            expected.iter().find_map(|(key, value)| {
                json_difference(&format!("{path}.{key}"), value, &actual[key])
            })
        }
        (Value::Array(expected), Value::Array(actual)) => {
            if expected.len() != actual.len() {
                return Some(format!(
                    "{path}: expected {} items, got {}",
                    expected.len(),
                    actual.len()
                ));
            }
            expected
                .iter()
                .zip(actual)
                .enumerate()
                .find_map(|(index, (expected, actual))| {
                    json_difference(&format!("{path}[{index}]"), expected, actual)
                })
        }
        _ if expected == actual => None,
        _ => Some(format!("{path}: expected {expected}, got {actual}")),
    }
}

/// The keys of an object in order, at every level of nested objects
/// (arrays contribute their first element), for the key order checks.
pub fn key_paths(value: &Value) -> Vec<String> {
    fn walk(prefix: &str, value: &Value, out: &mut Vec<String>) {
        match value {
            Value::Object(map) => {
                for (key, value) in map {
                    let path = format!("{prefix}.{key}");
                    out.push(path.clone());
                    walk(&path, value, out);
                }
            }
            Value::Array(items) => {
                if let Some(first) = items.first() {
                    walk(&format!("{prefix}[]"), first, out);
                }
            }
            _ => {}
        }
    }
    let mut out = Vec::new();
    walk("$", value, &mut out);
    out
}
