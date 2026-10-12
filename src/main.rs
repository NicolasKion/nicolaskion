#[tokio::main]
async fn main() {
    // Structured logs; tune with RUST_LOG (e.g. RUST_LOG=nicolaskion=debug).
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    // Local configuration from .env, if present; the real environment wins.
    dotenvy::dotenv().ok();

    let config = nicolaskion::config::Config::from_env();
    let addr = nicolaskion::server::bind_addr();
    let state = nicolaskion::server::AppState::new(config);

    // The GitHub and ESI data are fetched now, so the first visitor after
    // a deploy does not wait for them.
    state.warm();

    let app = nicolaskion::server::router(state);

    tracing::info!("listening on http://{addr}");
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("bind address");
    axum::serve(listener, app.into_make_service())
        .await
        .expect("serve");
}
