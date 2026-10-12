//! Downloads the featured project logos from their sources into
//! `frontend/static/logos` (or `$STATIC_DIR`). Exits 1 when any logo could
//! not be synced; those keep their current file.
//!
//!     cargo run --bin sync_logos

use std::path::PathBuf;
use std::process::ExitCode;

use nicolaskion::config::UpstreamConfig;
use nicolaskion::featured::PROJECTS;
use nicolaskion::upstream::Upstream;

#[tokio::main]
async fn main() -> ExitCode {
    let static_dir = std::env::var("STATIC_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("frontend/static"));
    let upstream = Upstream::new(UpstreamConfig::default());

    let results = nicolaskion::logos::sync(&PROJECTS, &static_dir, &upstream).await;
    let mut failed = false;
    for result in &results {
        match &result.error {
            Some(error) => {
                failed = true;
                eprintln!("ERROR {}: {error}", result.path);
            }
            None => println!("INFO {} synced from {}", result.path, result.source),
        }
    }
    if failed {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
