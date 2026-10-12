//! The non-Inertia answers against what Laravel sent: the health page,
//! the error pages and the dead storage routes.

use axum::http::{Method, StatusCode};

use crate::common::TestApp;
use crate::common::golden;

/// Laravel's health route: the same HTML page, byte for byte apart from
/// the render time.
#[tokio::test]
async fn up_answers_the_health_page() {
    let recorded = golden::http("up");
    assert_eq!(recorded["status"], 200);
    let expected = golden::http_body("up");

    let app = TestApp::new().await;
    let response = app.get("/up").await;
    assert_eq!(response.status, StatusCode::OK);
    assert_eq!(
        response.header("content-type"),
        Some("text/html; charset=utf-8")
    );
    let render_time = |text: &str| {
        let start = text.find("rendered in ").expect("render time") + "rendered in ".len();
        let end = start + text[start..].find("ms").expect("ms");
        format!("{}N{}", &text[..start], &text[end..])
    };
    assert_eq!(render_time(&response.text()), render_time(&expected));
}

/// Unknown paths outside /api get Laravel's minimal 404 page, byte for
/// byte.
#[tokio::test]
async fn unknown_paths_answer_the_not_found_page() {
    let recorded = golden::http("unknown-path");
    assert_eq!(recorded["status"], 404);

    let app = TestApp::new().await;
    let response = app.get("/nope").await;
    assert_eq!(response.status, StatusCode::NOT_FOUND);
    assert_eq!(
        response.header("content-type"),
        Some("text/html; charset=utf-8")
    );
    assert_eq!(response.text(), golden::http_body("unknown-path"));
}

/// Laravel's local disk routes refuse every unsigned request with its 403
/// page; nothing ever signed one.
#[tokio::test]
async fn storage_routes_answer_forbidden() {
    let app = TestApp::new().await;
    for (name, method, path) in [
        (
            "storage-get",
            Method::GET,
            "/storage/tt-8d20886697466b8297bb6d6e1735fa53/missing.json",
        ),
        ("storage-put", Method::PUT, "/storage/upload.txt"),
    ] {
        let recorded = golden::http(name);
        assert_eq!(recorded["status"], 403, "{name}");
        let response = app.request(method, path, &[]).await;
        assert_eq!(response.status, StatusCode::FORBIDDEN, "{name}");
        assert_eq!(response.text(), golden::http_body(name), "{name}");
    }
}
