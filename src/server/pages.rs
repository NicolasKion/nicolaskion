//! The props of the one page, as the Laravel Inertia page received them
//! with the shared props merged in. SvelteKit's `+page.server.ts` loads
//! them.

use axum::response::Response;

use super::errors::not_implemented;

pub async fn home() -> Response {
    not_implemented()
}
