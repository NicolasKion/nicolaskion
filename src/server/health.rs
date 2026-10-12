//! Health checks: Laravel's `/up` page and the deploy check `/api/health`.

use axum::response::Response;

use super::errors::not_implemented;

pub async fn up() -> Response {
    not_implemented()
}

pub async fn health() -> Response {
    not_implemented()
}
