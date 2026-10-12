//! The route contract of the paths the api owns: health checks, error
//! answers, the dead storage routes and tabs left open on the Laravel
//! site.

use axum::http::{Method, StatusCode};
use serde_json::json;

use crate::common::TestApp;

#[tokio::test]
async fn health_reports_ok() {
    let app = TestApp::new().await;
    let response = app.get("/api/health").await;
    assert_eq!(response.status, StatusCode::OK);
    assert_eq!(response.json(), json!({ "status": "ok" }));
}

#[tokio::test]
async fn up_names_the_app() {
    let app = TestApp::with(|config| config.app_name = "A <b> & C".to_owned()).await;
    let response = app.get("/up").await;
    assert_eq!(response.status, StatusCode::OK);
    assert!(
        response
            .text()
            .contains("<title>A &lt;b&gt; &amp; C</title>"),
        "{}",
        response.text()
    );
    assert!(response.text().contains("Application up"));

    let head = app.request(Method::HEAD, "/up", &[]).await;
    assert_eq!(head.status, StatusCode::OK);
}

#[tokio::test]
async fn unknown_api_paths_answer_json() {
    let app = TestApp::new().await;
    for path in ["/api", "/api/pages/nothing", "/api/pages/projects"] {
        let response = app.get(path).await;
        assert_eq!(response.status, StatusCode::NOT_FOUND, "{path}");
        assert_eq!(response.json(), json!({ "message": "Not Found" }), "{path}");
    }
}

/// The local disk routes took GET, HEAD and PUT; anything else was 405.
#[tokio::test]
async fn storage_routes_refuse_every_request() {
    let app = TestApp::new().await;
    for method in [Method::GET, Method::HEAD, Method::PUT] {
        let response = app
            .request(method.clone(), "/storage/anything/at/all.txt", &[])
            .await;
        assert_eq!(response.status, StatusCode::FORBIDDEN, "{method}");
    }
    let response = app.request(Method::POST, "/storage/file.txt", &[]).await;
    assert_eq!(response.status, StatusCode::METHOD_NOT_ALLOWED);
    assert!(response.text().contains("Method Not Allowed"));
    assert_eq!(app.get("/storage").await.status, StatusCode::NOT_FOUND);
}

/// FIXES.md (deliberate changes): a request from a tab left open on the
/// Laravel site carries `X-Inertia`; the 409 makes Inertia reload the
/// page in full.
#[tokio::test]
async fn an_inertia_request_reloads_the_page() {
    let app = TestApp::new().await;
    let response = app
        .get_with(
            "/api/pages/home?x=1",
            &[("x-inertia", "true"), ("host", "nicolaskion.dev")],
        )
        .await;
    assert_eq!(response.status, StatusCode::CONFLICT);
    assert_eq!(
        response.header("x-inertia-location"),
        Some("https://nicolaskion.dev/api/pages/home?x=1")
    );
    assert_eq!(app.upstream.requests(), []);
}
