//! `/storage/{path}`: Laravel's local disk routes, which only answer signed
//! URLs, and nothing ever signed one. The update feed under
//! `/storage/tt-.../` never reaches the api: Caddy serves it from disk.

use axum::http::{StatusCode, header};
use axum::response::Response;

use super::errors::error_page;

/// GET, HEAD and PUT: the 403 Laravel gave every unsigned request.
pub async fn local_disk() -> Response {
    error_page(StatusCode::FORBIDDEN)
}

/// Any other method.
pub async fn method_not_allowed() -> Response {
    let mut response = error_page(StatusCode::METHOD_NOT_ALLOWED);
    response.headers_mut().insert(
        header::ALLOW,
        header::HeaderValue::from_static("GET, HEAD, PUT"),
    );
    response
}
