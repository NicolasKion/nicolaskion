//! What the api sends GitHub and ESI: headers, query strings, paging and
//! retries, against the mock upstream.

use std::time::Duration;

use serde_json::{Value, json};

use nicolaskion::esi::COMPATIBILITY_DATE;
use nicolaskion::upstream::USER_AGENT;

use crate::common::{Service, TestApp};

fn repository(id: usize) -> Value {
    let mut repository = crate::common::upstream::fixture("github/repos.json")[0].clone();
    repository["id"] = json!(id);
    repository["full_name"] = json!(format!("NicolasKion/repo-{id}"));
    repository
}

/// The token goes out as a bearer token with GitHub's media type, and
/// the repositories are asked for newest push first, owned and through
/// organizations, 100 at a time.
#[tokio::test]
async fn github_requests_carry_the_token_and_the_listing_options() {
    let app = TestApp::with(|config| config.esi.character_id = None).await;
    app.home().await;

    let requests = app.upstream.requests_to(Service::GitHub);
    assert_eq!(requests.len(), 2, "{requests:?}");
    for request in &requests {
        assert_eq!(request.authorization.as_deref(), Some("Bearer fake-token"));
        assert_eq!(
            request.accept.as_deref(),
            Some("application/vnd.github+json")
        );
        assert_eq!(request.user_agent.as_deref(), Some(USER_AGENT));
    }
    let repos = requests
        .iter()
        .find(|request| request.path == "/github/user/repos")
        .expect("repositories requested");
    let query: Vec<(&str, &str)> = repos
        .query
        .iter()
        .map(|(key, value)| (key.as_str(), value.as_str()))
        .collect();
    assert_eq!(
        query,
        [
            ("affiliation", "owner,organization_member"),
            ("page", "1"),
            ("per_page", "100"),
            ("sort", "pushed"),
        ]
    );
    assert!(
        requests
            .iter()
            .any(|request| request.path == "/github/user")
    );
}

/// ESI is asked on its versionless routes with a pinned compatibility
/// date and the app's User-Agent; no token goes to CCP.
#[tokio::test]
async fn esi_requests_use_the_compatibility_date() {
    let app = TestApp::with(|config| config.github.token = None).await;
    app.home().await;

    let requests = app.upstream.requests_to(Service::Esi);
    let paths: Vec<&str> = requests
        .iter()
        .map(|request| request.path.as_str())
        .collect();
    assert_eq!(
        paths,
        [
            "/esi/characters/95569887",
            "/esi/corporations/98630705",
            "/esi/alliances/99014466"
        ]
    );
    for request in &requests {
        assert_eq!(
            request.compatibility_date.as_deref(),
            Some(COMPATIBILITY_DATE)
        );
        assert_eq!(request.accept.as_deref(), Some("application/json"));
        assert_eq!(request.user_agent.as_deref(), Some(USER_AGENT));
        assert_eq!(request.authorization, None);
    }
}

/// Paging stops at the first page shorter than 100; a full last page
/// costs one more request for the empty page after it.
#[tokio::test]
async fn paging_stops_at_a_short_page() {
    for (count, pages) in [(0, 1), (99, 1), (100, 2), (200, 3), (250, 3)] {
        let app = TestApp::with(|config| config.esi.character_id = None).await;
        app.upstream.change(|data| {
            data.repositories = (1..=count).map(repository).collect();
        });
        let page = app.home().await;
        assert_eq!(
            page["repositories"].as_array().expect("repositories").len(),
            count,
            "{count}"
        );
        let requested = app
            .upstream
            .requests_to(Service::GitHub)
            .into_iter()
            .filter(|request| request.path == "/github/user/repos")
            .count();
        assert_eq!(requested, pages, "{count} repositories");
    }
}

/// Two failures are retried; the third attempt answers the page.
#[tokio::test]
async fn failed_requests_are_retried_twice() {
    let app = TestApp::with(|config| config.github.token = None).await;
    app.upstream.fail_next(Service::Esi, 503, 2);

    assert_eq!(app.home().await["character"]["name"], "Nicolas Kion");
    let characters = app
        .upstream
        .requests_to(Service::Esi)
        .into_iter()
        .filter(|request| request.path.starts_with("/esi/characters/"))
        .count();
    assert_eq!(characters, 3);
}

/// After three failed attempts the request gives up.
#[tokio::test]
async fn requests_give_up_after_three_attempts() {
    let app = TestApp::with(|config| config.esi.character_id = None).await;
    app.upstream.fail_with(Service::GitHub, 502);

    app.home().await;
    let repos = app
        .upstream
        .requests_to(Service::GitHub)
        .into_iter()
        .filter(|request| request.path == "/github/user/repos")
        .count();
    assert_eq!(repos, 3);
}

/// The retries wait the configured delays (250 ms and 1 s in production).
#[tokio::test]
async fn retries_wait_between_attempts() {
    let app = TestApp::with(|config| {
        config.github.token = None;
        config.upstream.retry_delays = vec![Duration::from_millis(150), Duration::from_millis(150)];
    })
    .await;
    app.upstream.fail_next(Service::Esi, 500, 2);

    let started = std::time::Instant::now();
    app.home().await;
    assert!(
        started.elapsed() >= Duration::from_millis(300),
        "{:?}",
        started.elapsed()
    );
}

/// A server that does not answer in time counts as a failure.
#[tokio::test]
async fn an_unreachable_upstream_counts_as_a_failure() {
    let app = TestApp::with(|config| {
        config.github.api_url = "http://127.0.0.1:9".to_owned();
        config.esi.base_url = "http://127.0.0.1:9".to_owned();
    })
    .await;
    let page = app.home().await;
    assert_eq!(page["profile"], Value::Null);
    assert_eq!(page["character"], Value::Null);
}
