//! The page props against the golden props Laravel's Inertia page
//! received, one test per recorded configuration and upstream scenario,
//! plus the requests Laravel sent upstream for each. Key sets are compared
//! at every level; the only differences allowed are the normalizations in
//! `common::golden` and the FIXES.md entries named below.

use axum::http::StatusCode;
use serde_json::{Value, json};

use crate::common::golden::{self, normalize_actual};
use crate::common::{TestApp, assert_json_eq, key_paths};

/// The app as the reference ran for a recorded page: the configuration
/// (record.py's SCENARIOS) and the mock upstream's scenario.
async fn app_for(name: &str) -> TestApp {
    let (token, character, scenario) = match name {
        "unconfigured" => (None, None, "normal"),
        "blank-config" => (Some("   "), Some(" "), "normal"),
        "full" => (Some("golden-token"), Some("95569887"), "normal"),
        "no-character" => (Some("golden-token"), None, "normal"),
        "no-alliance" => (Some("golden-token"), Some("95569887"), "no-alliance"),
        "esi-down" => (Some("golden-token"), Some("95569887"), "esi-down"),
        "paginated" => (Some("golden-token"), None, "paginated"),
        "github-down" => (Some("golden-token"), Some("95569887"), "github-down"),
        other => panic!("no configuration for the golden page {other}"),
    };
    let app = TestApp::with(|config| {
        config.github.token = token.map(str::to_owned);
        config.esi.character_id = character.map(str::to_owned);
    })
    .await;
    app.upstream.scenario(scenario);
    app
}

async fn props(app: &TestApp) -> Value {
    let mut props = app.home().await;
    normalize_actual(&mut props);
    props
}

/// Requests the page's props and compares them with the golden fixture.
async fn assert_matches_golden(name: &str) {
    let golden = golden::page(name);
    assert_eq!(golden.status, 200, "{name} was not recorded as a page");
    assert_eq!(golden.component.as_deref(), Some("Projects"));
    let app = app_for(name).await;
    assert_json_eq(&golden.props.expect("props"), &props(&app).await);
}

macro_rules! golden_pages {
    ($($test:ident => $name:literal,)*) => {
        $(
            #[tokio::test]
            async fn $test() {
                assert_matches_golden($name).await;
            }
        )*

        /// Every recorded page has its test above.
        #[test]
        fn every_golden_page_has_a_test() {
            let tested = [$($name,)* "github-down"];
            for name in golden::page_names() {
                assert!(tested.contains(&name.as_str()), "no test for golden page {name}");
            }
        }
    };
}

golden_pages! {
    unconfigured_matches_laravel => "unconfigured",
    blank_config_matches_laravel => "blank-config",
    full_matches_laravel => "full",
    no_character_matches_laravel => "no-character",
    no_alliance_matches_laravel => "no-alliance",
    esi_down_matches_laravel => "esi-down",
    paginated_matches_laravel => "paginated",
}

/// FIXES.md "A GitHub failure with nothing cached was a 500": Laravel
/// answered 500; the page now renders with everything GitHub feeds left
/// empty (as without a token) and the dossier from ESI.
#[tokio::test]
async fn github_down_renders_without_github_instead_of_failing() {
    let recorded = golden::page("github-down");
    assert_eq!(recorded.status, 500);

    let app = app_for("github-down").await;
    let response = app.get("/api/pages/home").await;
    assert_eq!(response.status, StatusCode::OK);
    let mut actual = response.json();
    assert_eq!(actual["githubConfigured"], json!(true));
    normalize_actual(&mut actual);

    let mut expected = golden::page("unconfigured").props.expect("props");
    expected["character"] = golden::page("full").props.expect("props")["character"].clone();
    assert_json_eq(&expected, &actual);
}

/// The props come in Laravel's order, keys of nested objects included.
#[tokio::test]
async fn props_keep_laravels_key_order() {
    let expected = golden::page("full").props.expect("props");
    let app = app_for("full").await;
    assert_eq!(key_paths(&props(&app).await), key_paths(&expected));
}

/// The new prop sits with the GitHub props.
#[tokio::test]
async fn github_configured_follows_the_token() {
    for (name, configured) in [
        ("full", true),
        ("unconfigured", false),
        ("blank-config", false),
    ] {
        let app = app_for(name).await;
        assert_eq!(
            app.home().await["githubConfigured"],
            json!(configured),
            "{name}"
        );
    }
}

/// What Laravel asked GitHub and ESI for while rendering each page:
/// the same paths (N3), query strings, Accept and Authorization headers,
/// and the same number of requests (retries included). The order may
/// differ: GitHub and ESI are asked concurrently now.
#[tokio::test]
async fn upstream_requests_match_laravel() {
    for name in golden::page_names() {
        if name == "github-down" {
            // Laravel stopped at the first GitHub error; covered by
            // `upstream_requests::*`.
            continue;
        }
        let app = app_for(&name).await;
        app.home().await;
        let mut actual: Vec<Value> = app
            .upstream
            .requests()
            .into_iter()
            .map(|request| {
                json!({
                    "path": request.path,
                    "query": request.query,
                    "accept": request.accept,
                    "authorization": request.authorization,
                })
            })
            .collect();
        let mut expected = golden::requests(&name);
        let sort_key = |value: &Value| value.to_string();
        actual.sort_by_key(sort_key);
        expected.sort_by_key(sort_key);
        assert_eq!(actual, expected, "{name}");
    }
}
