//! Health checks: Laravel's `/up` page and the deploy check `/api/health`.

use std::time::Instant;

use axum::Json;
use axum::extract::State;
use axum::response::Html;
use serde_json::{Value, json};

use super::AppState;

/// Laravel's health page (`withRouting(health: '/up')`).
const UP_PAGE: &str = include_str!("up.html");

/// PHP's `htmlspecialchars`, as Blade's `{{ }}` escaped.
pub fn escape_html(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#039;")
}

pub async fn up(State(state): State<AppState>) -> Html<String> {
    let started = Instant::now();
    Html(
        UP_PAGE
            .replace("{{app_name}}", &escape_html(&state.config.app_name))
            .replace(
                "{{milliseconds}}",
                &started.elapsed().as_millis().to_string(),
            ),
    )
}

/// The api is up. It has no database or other dependency to check.
pub async fn health() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}
