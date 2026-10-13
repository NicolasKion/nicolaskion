//! Error answers: JSON under `/api`, a minimal error page
//! everywhere else.

use axum::Json;
use axum::http::{StatusCode, Uri};
use axum::response::{Html, IntoResponse, Response};
use serde_json::json;

/// The minimal error view with the status and its message.
const ERROR_PAGE: &str = include_str!("error.html");

pub fn json_error(status: StatusCode, message: &str) -> Response {
    (status, Json(json!({ "message": message }))).into_response()
}

pub fn error_page(status: StatusCode) -> Response {
    let message = match status {
        StatusCode::INTERNAL_SERVER_ERROR => "Server Error",
        status => status.canonical_reason().unwrap_or("Error"),
    };
    let page = ERROR_PAGE
        .replace("{{status}}", status.as_str())
        .replace("{{message}}", message);
    (status, Html(page)).into_response()
}

pub fn not_implemented() -> Response {
    json_error(StatusCode::NOT_IMPLEMENTED, "Not Implemented")
}

/// Unknown paths: JSON for the api, the 404 page otherwise.
pub async fn not_found(uri: Uri) -> Response {
    if uri.path() == "/api" || uri.path().starts_with("/api/") {
        json_error(StatusCode::NOT_FOUND, "Not Found")
    } else {
        error_page(StatusCode::NOT_FOUND)
    }
}
