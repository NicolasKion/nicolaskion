//! `/storage/{path}`: Laravel's local disk routes, which only answer signed
//! URLs and nothing ever signed one. The update feed under
//! `/storage/tt-.../` never reaches the api: Caddy serves it from disk.

use axum::response::Response;

use super::errors::not_implemented;

pub async fn local_disk() -> Response {
    not_implemented()
}
