//! The HTTP side: shared state, the router and the cross-cutting layers.
//! The page is rendered by SvelteKit; the api serves its props as JSON
//! (`/api/pages/home`), the health checks and the dead storage routes.

pub mod errors;
pub mod health;
pub mod pages;
pub mod storage;

use std::sync::Arc;

use axum::Router;
use axum::extract::{Request, State};
use axum::http::{Method, StatusCode, header};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::routing::get;

use crate::cache::Clock;
use crate::config::Config;
use crate::esi::Esi;
use crate::github::GitHub;
use crate::upstream::Upstream;

/// The default bind address of the api; the SvelteKit dev proxy and the
/// reverse proxy point at it.
const DEFAULT_BIND_ADDR: &str = "127.0.0.1:3000";

/// The listen address from `BIND_ADDR`, with the local default.
pub fn bind_addr() -> String {
    std::env::var("BIND_ADDR").unwrap_or_else(|_| DEFAULT_BIND_ADDR.to_owned())
}

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub github: GitHub,
    pub esi: Esi,
}

impl AppState {
    pub fn new(config: Config) -> Self {
        Self::with_clock(config, Clock::default())
    }

    /// The state with the clock the caches read (tests move it).
    pub fn with_clock(config: Config, clock: Clock) -> Self {
        let upstream = Upstream::new(config.upstream.clone());
        Self {
            github: GitHub::new(config.github.clone(), upstream.clone(), clock.clone()),
            esi: Esi::new(config.esi.clone(), upstream, clock),
            config: Arc::new(config),
        }
    }

    /// Fetches the GitHub and ESI data in the background, so the first
    /// visitor finds it cached.
    pub fn warm(&self) {
        if self.github.is_configured() {
            let github = self.github.clone();
            tokio::spawn(async move { github.profile().await });
            let github = self.github.clone();
            tokio::spawn(async move { github.repositories().await });
        }
        if self.esi.is_configured() {
            let esi = self.esi.clone();
            tokio::spawn(async move { esi.character().await });
        }
    }
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/up", get(health::up))
        .route(
            "/storage/{*path}",
            get(storage::local_disk)
                .put(storage::local_disk)
                .fallback(storage::method_not_allowed),
        )
        .nest("/api", api_router())
        .fallback(errors::not_found)
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            reload_inertia_requests,
        ))
        .with_state(state)
}

fn api_router() -> Router<AppState> {
    Router::new()
        .route("/health", get(health::health))
        .route("/pages/home", get(pages::home))
}

/// Tabs left open on the Laravel site still send Inertia's XHR
/// (`X-Inertia: true`). A 409 with `X-Inertia-Location` makes Inertia load
/// that URL in full: the same URL for a GET, the page the request came
/// from (its Referer, or the home page) for anything else.
async fn reload_inertia_requests(
    State(state): State<AppState>,
    request: Request,
    next: Next,
) -> Response {
    if !request.headers().contains_key("x-inertia") {
        return next.run(request).await;
    }
    let headers = request.headers();
    let location = if matches!(*request.method(), Method::GET | Method::HEAD) {
        let scheme = headers
            .get("x-forwarded-proto")
            .and_then(|value| value.to_str().ok())
            .unwrap_or_else(|| state.config.app_url.split("://").next().unwrap_or("https"));
        let host = headers
            .get(header::HOST)
            .and_then(|value| value.to_str().ok())
            .unwrap_or_default();
        let path = request
            .uri()
            .path_and_query()
            .map_or("/", |path| path.as_str());
        format!("{scheme}://{host}{path}")
    } else {
        headers
            .get(header::REFERER)
            .and_then(|value| value.to_str().ok())
            .filter(|referer| referer.starts_with(&state.config.app_url))
            .map_or_else(|| format!("{}/", state.config.app_url), str::to_owned)
    };
    let mut response = StatusCode::CONFLICT.into_response();
    if let Ok(location) = location.parse() {
        response
            .headers_mut()
            .insert("x-inertia-location", location);
    }
    response
}
