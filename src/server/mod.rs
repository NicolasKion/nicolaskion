//! The HTTP side: shared state, the router and the cross-cutting layers.
//! The page is rendered by SvelteKit; the api serves its props as JSON
//! (`/api/pages/home`), the health checks and the dead storage routes.

pub mod errors;
pub mod health;
pub mod pages;
pub mod storage;

use std::sync::Arc;

use axum::Router;
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
        .with_state(state)
}

fn api_router() -> Router<AppState> {
    Router::new()
        .route("/health", get(health::health))
        .route("/pages/home", get(pages::home))
}
