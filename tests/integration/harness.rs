//! The harness itself: the mock upstream and the route inventory.

use axum::http::{Method, StatusCode};
use serde_json::Value;

use crate::common::{MockUpstream, Service, TestApp};

async fn get(url: &str) -> (u16, Value) {
    let response = reqwest::get(url).await.expect("mock answers");
    let status = response.status().as_u16();
    (status, response.json().await.unwrap_or(Value::Null))
}

/// The mock serves the fixture files, pages the repositories like GitHub
/// and records what it was asked.
#[tokio::test]
async fn the_mock_upstream_serves_the_fixtures() {
    let mock = MockUpstream::start().await;
    let (status, user) = get(&format!("{}/user", mock.github_url())).await;
    assert_eq!(status, 200);
    assert_eq!(user["login"], "NicolasKion");

    let (_, page) = get(&format!(
        "{}/user/repos?per_page=5&page=2",
        mock.github_url()
    ))
    .await;
    let ids: Vec<i64> = page
        .as_array()
        .expect("page")
        .iter()
        .map(|repository| repository["id"].as_i64().expect("id"))
        .collect();
    assert_eq!(ids, [106, 107, 108, 109, 110]);

    let (_, corporation) = get(&format!("{}/corporations/98630705", mock.esi_url())).await;
    assert_eq!(corporation["ticker"], "OWL.");
    let (_, character) = get(&format!("{}/latest/characters/1/", mock.esi_url())).await;
    assert_eq!(character["name"], "Nicolas Kion");

    let paths: Vec<String> = mock.requests().into_iter().map(|r| r.path).collect();
    assert_eq!(
        paths,
        [
            "/github/user",
            "/github/user/repos",
            "/esi/corporations/98630705",
            "/esi/latest/characters/1/"
        ]
    );
    assert_eq!(mock.requests()[1].query["page"], "2");
}

/// Failures are per service, for good or for a number of requests.
#[tokio::test]
async fn the_mock_upstream_fails_on_demand() {
    let mock = MockUpstream::start().await;
    mock.fail_with(Service::GitHub, 502);
    assert_eq!(get(&format!("{}/user", mock.github_url())).await.0, 502);
    assert_eq!(get(&format!("{}/alliances/1", mock.esi_url())).await.0, 200);
    mock.recover(Service::GitHub);
    assert_eq!(get(&format!("{}/user", mock.github_url())).await.0, 200);

    mock.fail_next(Service::Esi, 503, 2);
    assert_eq!(get(&format!("{}/alliances/1", mock.esi_url())).await.0, 503);
    assert_eq!(get(&format!("{}/alliances/1", mock.esi_url())).await.0, 503);
    assert_eq!(get(&format!("{}/alliances/1", mock.esi_url())).await.0, 200);
}

/// Every route from the Laravel inventory that the api owns exists: it
/// may not work yet, but it is never an unknown path.
#[tokio::test]
async fn every_api_route_exists() {
    let app = TestApp::new().await;
    for (method, path) in [
        (Method::GET, "/up"),
        (Method::HEAD, "/up"),
        (Method::GET, "/api/health"),
        (Method::GET, "/api/pages/home"),
        (Method::GET, "/storage/some/file.txt"),
        (Method::PUT, "/storage/some/file.txt"),
    ] {
        let response = app.request(method.clone(), path, &[]).await;
        assert_ne!(response.status, StatusCode::NOT_FOUND, "{method} {path}");
    }
}
