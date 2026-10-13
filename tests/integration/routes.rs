//! The route contract of the paths the api owns: health checks, error
//! answers and the storage routes.

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
    assert_eq!(
        response.header("content-type"),
        Some("text/html; charset=utf-8")
    );

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

/// GET, HEAD and PUT are forbidden, any other method is not allowed.
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

/// Unknown paths outside /api get the minimal HTML 404 page.
#[tokio::test]
async fn unknown_paths_answer_the_not_found_page() {
    let app = TestApp::new().await;
    let response = app.get("/nope").await;
    assert_eq!(response.status, StatusCode::NOT_FOUND);
    assert_eq!(
        response.header("content-type"),
        Some("text/html; charset=utf-8")
    );
    let text = response.text();
    assert!(text.starts_with("<!DOCTYPE html>"), "{text}");
    assert!(text.contains("<title>Not Found</title>"), "{text}");
    assert!(text.contains("404                    </h1>"), "{text}");
    assert!(
        text.contains("Not Found                    </div>"),
        "{text}"
    );
}

/// The storage routes answer the HTML 403 page.
#[tokio::test]
async fn storage_routes_answer_the_forbidden_page() {
    let app = TestApp::new().await;
    for (method, path) in [
        (
            Method::GET,
            "/storage/tt-8d20886697466b8297bb6d6e1735fa53/missing.json",
        ),
        (Method::PUT, "/storage/upload.txt"),
    ] {
        let response = app.request(method.clone(), path, &[]).await;
        assert_eq!(response.status, StatusCode::FORBIDDEN, "{method}");
        assert_eq!(
            response.header("content-type"),
            Some("text/html; charset=utf-8"),
            "{method}"
        );
        let text = response.text();
        assert!(text.contains("<title>Forbidden</title>"), "{method}");
        assert!(text.contains("403                    </h1>"), "{method}");
    }
}
