//! The stale-while-revalidate caches: GitHub 10 minutes fresh and an hour
//! stale, ESI an hour fresh and a day stale. The test clock steps through
//! the windows.

use std::time::Duration;

use serde_json::{Value, json};

use crate::common::{Service, TestApp};

const MINUTE: Duration = Duration::from_secs(60);

fn github_requests(app: &TestApp) -> usize {
    app.upstream.requests_to(Service::GitHub).len()
}

fn esi_requests(app: &TestApp) -> usize {
    app.upstream.requests_to(Service::Esi).len()
}

/// Within the fresh window the page is served from the cache.
#[tokio::test]
async fn fresh_data_is_served_from_the_cache() {
    let app = TestApp::new().await;
    app.home().await;
    let (github, esi) = (github_requests(&app), esi_requests(&app));
    assert_eq!((github, esi), (2, 3));

    app.clock.advance(9 * MINUTE);
    app.upstream
        .change(|data| data.user["login"] = json!("Changed"));
    let page = app.home().await;
    assert_eq!(page["profile"]["login"], "NicolasKion");
    assert_eq!((github_requests(&app), esi_requests(&app)), (2, 3));
}

/// Past the fresh window the cached data answers at once and a
/// background refresh replaces it for the next visitor.
#[tokio::test]
async fn stale_data_is_served_while_it_refreshes() {
    let app = TestApp::new().await;
    app.home().await;

    app.clock.advance(11 * MINUTE);
    app.upstream
        .change(|data| data.user["login"] = json!("Changed"));
    let page = app.home().await;
    assert_eq!(page["profile"]["login"], "NicolasKion");
    app.wait_for_requests(Service::GitHub, 4).await;

    assert_eq!(app.home().await["profile"]["login"], "Changed");
    // ESI was still fresh.
    assert_eq!(esi_requests(&app), 3);
}

/// Past the stale window the data is fetched before answering.
#[tokio::test]
async fn expired_data_is_fetched_before_answering() {
    let app = TestApp::new().await;
    app.home().await;

    app.clock.advance(61 * MINUTE);
    app.upstream
        .change(|data| data.user["login"] = json!("Changed"));
    assert_eq!(app.home().await["profile"]["login"], "Changed");
}

/// The ESI windows: an hour fresh, a day stale.
#[tokio::test]
async fn the_character_is_cached_an_hour_and_kept_a_day() {
    let app = TestApp::with(|config| config.github.token = None).await;
    app.home().await;
    assert_eq!(esi_requests(&app), 3);

    app.clock.advance(59 * MINUTE);
    app.home().await;
    assert_eq!(esi_requests(&app), 3);

    app.clock.advance(2 * MINUTE);
    app.upstream
        .change(|data| data.character["name"] = json!("Renamed"));
    assert_eq!(app.home().await["character"]["name"], "Nicolas Kion");
    app.wait_for_requests(Service::Esi, 6).await;
    assert_eq!(app.home().await["character"]["name"], "Renamed");

    app.clock.advance(Duration::from_secs(86_401));
    app.upstream
        .change(|data| data.character["name"] = json!("Again"));
    assert_eq!(app.home().await["character"]["name"], "Again");
}

/// A failed background refresh keeps the stale data and tries again on
/// a later visit.
#[tokio::test]
async fn a_failed_refresh_keeps_the_stale_data() {
    let app = TestApp::new().await;
    app.home().await;

    app.clock.advance(11 * MINUTE);
    app.upstream.fail_with(Service::GitHub, 500);
    app.home().await;
    app.wait_for_requests(Service::GitHub, 2 + 3).await;
    let page = app.home().await;
    assert_eq!(page["profile"]["login"], "NicolasKion");
    assert_eq!(
        page["repositories"].as_array().expect("repositories").len(),
        12
    );
}

/// Past the stale window a failing upstream does not empty the page: the
/// last data is served until a fetch succeeds.
#[tokio::test]
async fn expired_data_is_served_when_the_refresh_fails() {
    let app = TestApp::new().await;
    app.home().await;

    app.clock.advance(Duration::from_secs(2 * 86_400));
    app.upstream.fail_with(Service::GitHub, 500);
    app.upstream.fail_with(Service::Esi, 503);
    let page = app.home().await;
    assert_eq!(page["profile"]["login"], "NicolasKion");
    assert_eq!(
        page["repositories"].as_array().expect("repositories").len(),
        12
    );
    assert_eq!(page["character"]["name"], "Nicolas Kion");

    app.upstream.recover(Service::GitHub);
    app.upstream
        .change(|data| data.user["login"] = json!("Back"));
    assert_eq!(app.home().await["profile"]["login"], "Back");
}

/// A failure is not cached: the next visit asks again.
#[tokio::test]
async fn a_failure_is_not_cached() {
    let app = TestApp::new().await;
    app.upstream.fail_with(Service::GitHub, 500);
    assert_eq!(app.home().await["profile"], Value::Null);

    app.upstream.recover(Service::GitHub);
    assert_eq!(app.home().await["profile"]["login"], "NicolasKion");
}

/// An answer that does not have the expected shape is a failure, never
/// cached.
#[tokio::test]
async fn a_malformed_answer_is_not_cached() {
    let app = TestApp::new().await;
    app.upstream
        .change(|data| data.repositories = vec![json!({ "id": "not a repository" })]);
    let page = app.home().await;
    assert_eq!(page["repositories"], json!([]));
    assert_eq!(page["profile"]["login"], "NicolasKion");

    app.upstream.change(|data| {
        data.repositories = crate::common::upstream::fixture("github/repos.json")
            .as_array()
            .expect("repositories")
            .clone();
    });
    assert_eq!(
        app.home().await["repositories"]
            .as_array()
            .expect("repositories")
            .len(),
        12
    );
}

/// Visitors arriving together on a cold cache share one fetch.
#[tokio::test]
async fn concurrent_visitors_share_one_fetch() {
    let app = TestApp::new().await;
    let pages = futures_join(&app).await;
    for page in pages {
        assert_eq!(page["profile"]["login"], "NicolasKion");
    }
    assert_eq!((github_requests(&app), esi_requests(&app)), (2, 3));
}

async fn futures_join(app: &TestApp) -> Vec<Value> {
    let mut handles = Vec::new();
    for _ in 0..5 {
        let router = app.router.clone();
        handles.push(tokio::spawn(async move {
            use axum::body::Body;
            use http_body_util::BodyExt;
            use tower::ServiceExt;
            let response = router
                .oneshot(
                    axum::http::Request::get("/api/pages/home")
                        .body(Body::empty())
                        .expect("request"),
                )
                .await
                .expect("answer");
            let body = response
                .into_body()
                .collect()
                .await
                .expect("body")
                .to_bytes();
            serde_json::from_slice::<Value>(&body).expect("json")
        }));
    }
    let mut pages = Vec::new();
    for handle in handles {
        pages.push(handle.await.expect("task"));
    }
    pages
}

/// The startup warm-up fetches everything before the first visitor.
#[tokio::test]
async fn warming_fetches_everything_up_front() {
    let app = TestApp::new().await;
    app.state.warm();
    app.wait_for_requests(Service::Esi, 3).await;
    app.wait_for_requests(Service::GitHub, 2).await;

    app.home().await;
    assert_eq!((github_requests(&app), esi_requests(&app)), (2, 3));
}

/// Without configuration the warm-up asks nobody.
#[tokio::test]
async fn warming_without_configuration_asks_nobody() {
    let app = TestApp::with(|config| {
        config.github.token = None;
        config.esi.character_id = None;
    })
    .await;
    app.state.warm();
    tokio::time::sleep(Duration::from_millis(100)).await;
    assert_eq!(app.upstream.requests(), []);
}
