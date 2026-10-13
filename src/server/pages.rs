//! The props of the one page, with the app name and a user that is
//! always null. SvelteKit's `+page.server.ts` loads them.

use axum::Json;
use axum::extract::State;
use serde_json::{Value, json};

use super::AppState;
use crate::projects::{featured_projects, public_repositories, stack_languages};

/// GitHub and ESI are asked together. Whatever fails with nothing cached
/// is left out; the page always renders.
pub async fn home(State(state): State<AppState>) -> Json<Value> {
    let (profile, repositories, character) = tokio::join!(
        state.github.profile(),
        state.github.repositories(),
        state.esi.character(),
    );
    let all = repositories.unwrap_or_default();
    let public = public_repositories(&all);
    Json(json!({
        "name": state.config.app_name,
        "auth": { "user": null },
        "githubConfigured": state.github.is_configured(),
        "profile": profile,
        "character": character,
        "featured": featured_projects(&state.config.featured, &public),
        "repositories": public,
        "stack": stack_languages(&all),
    }))
}
