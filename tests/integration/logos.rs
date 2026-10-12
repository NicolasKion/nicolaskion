//! Port of tests/Feature/SyncLogosTest.php: `sync_logos` downloads every
//! logo and dark variant from its source into the static directory, and
//! keeps the existing file when a download fails or is not the expected
//! image type.

use nicolaskion::config::UpstreamConfig;
use nicolaskion::featured::FeaturedSource;
use nicolaskion::logos::{self, LogoResult};
use nicolaskion::upstream::Upstream;

use crate::common::MockUpstream;

const PNG_BYTES: &[u8] =
    b"\x89PNG\r\n\x1a\n\0\0\0\rIHDR\0\0\0\x01\0\0\0\x01\x08\x06\0\0\0\x1f\x15\xc4\x89";
const SVG_BYTES: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1 1"></svg>"#;

struct Setup {
    mock: MockUpstream,
    dir: tempfile::TempDir,
    projects: Vec<FeaturedSource>,
}

/// The suite's `beforeEach`: two projects, one with a dark variant, whose
/// sources live on the mock.
async fn setup() -> Setup {
    let mock = MockUpstream::start().await;
    let dir = tempfile::tempdir().expect("temp dir");
    std::fs::create_dir_all(dir.path().join("logos")).expect("logos dir");
    let leak = |text: String| -> &'static str { Box::leak(text.into_boxed_str()) };
    let projects = vec![
        FeaturedSource {
            logo: Some("/logos/telescope.png"),
            logo_source: Some(leak(mock.file_url("icon.png"))),
            ..FeaturedSource::new("Telescope", "")
        },
        FeaturedSource {
            logo: Some("/logos/wormholesystems.svg"),
            logo_source: Some(leak(mock.file_url("logo.svg"))),
            logo_dark: Some("/logos/wormholesystems-light.svg"),
            logo_dark_source: Some(leak(mock.file_url("logo-dark.svg"))),
            ..FeaturedSource::new("wormhole.systems", "")
        },
    ];
    Setup {
        mock,
        dir,
        projects,
    }
}

fn upstream() -> Upstream {
    Upstream::new(UpstreamConfig {
        retry_delays: vec![std::time::Duration::from_millis(1); 2],
        ..UpstreamConfig::default()
    })
}

impl Setup {
    fn read(&self, logo: &str) -> Vec<u8> {
        std::fs::read(self.dir.path().join(logo.trim_start_matches('/'))).expect("logo file")
    }

    async fn sync(&self) -> Vec<LogoResult> {
        logos::sync(&self.projects, self.dir.path(), &upstream()).await
    }
}

fn failed(results: &[LogoResult]) -> bool {
    results.iter().any(|result| result.error.is_some())
}

#[tokio::test]
async fn downloads_every_logo_and_dark_variant_from_its_source() {
    let setup = setup().await;
    setup.mock.serve_file("icon.png", 200, PNG_BYTES);
    setup.mock.serve_file("logo.svg", 200, SVG_BYTES);
    let dark = String::from_utf8_lossy(SVG_BYTES).replace("1 1", "2 2");
    setup.mock.serve_file("logo-dark.svg", 200, dark.as_bytes());

    let results = setup.sync().await;
    assert!(!failed(&results), "{results:?}");
    assert_eq!(results.len(), 3);
    assert_eq!(setup.read("/logos/telescope.png"), PNG_BYTES);
    assert_eq!(setup.read("/logos/wormholesystems.svg"), SVG_BYTES);
    assert!(
        String::from_utf8_lossy(&setup.read("/logos/wormholesystems-light.svg")).contains("2 2")
    );
}

/// The dataset of the PHP test: a missing source and a wrong image type.
#[tokio::test]
async fn keeps_the_existing_logo_when_the_source_is_missing_or_not_the_expected_image_type() {
    for (case, status, body, message) in [
        (
            "missing source",
            404,
            b"404: Not Found".as_slice(),
            "download failed with status 404",
        ),
        (
            "wrong image type",
            200,
            SVG_BYTES,
            "expected image/png but received image/svg+xml",
        ),
    ] {
        let setup = setup().await;
        std::fs::write(setup.dir.path().join("logos/telescope.png"), "existing").expect("write");
        setup.mock.serve_file("icon.png", status, body);
        setup.mock.serve_file("logo.svg", 200, SVG_BYTES);
        setup.mock.serve_file("logo-dark.svg", 200, SVG_BYTES);

        let results = setup.sync().await;
        assert!(failed(&results), "{case}");
        assert_eq!(setup.read("/logos/telescope.png"), b"existing", "{case}");
        assert!(
            setup.dir.path().join("logos/wormholesystems.svg").is_file(),
            "{case}"
        );
        let telescope = results
            .iter()
            .find(|result| result.path == "/logos/telescope.png")
            .expect("telescope result");
        assert_eq!(telescope.error.as_deref(), Some(message), "{case}");
    }
}

/// A logo shared by two projects is downloaded once.
#[tokio::test]
async fn a_shared_logo_is_synced_once() {
    let mut setup = setup().await;
    setup.projects.push(setup.projects[1]);
    setup.mock.serve_file("icon.png", 200, PNG_BYTES);
    setup.mock.serve_file("logo.svg", 200, SVG_BYTES);
    setup.mock.serve_file("logo-dark.svg", 200, SVG_BYTES);

    let results = setup.sync().await;
    let paths: Vec<&str> = results.iter().map(|result| result.path.as_str()).collect();
    assert_eq!(
        paths,
        [
            "/logos/telescope.png",
            "/logos/wormholesystems.svg",
            "/logos/wormholesystems-light.svg"
        ]
    );
    assert_eq!(setup.mock.requests().len(), 3);
}

/// Projects without a logo or a source are skipped.
#[tokio::test]
async fn logos_without_a_source_are_skipped() {
    let setup = setup().await;
    let projects = [
        FeaturedSource {
            logo: Some("/logos/telescope.png"),
            ..FeaturedSource::new("No source", "")
        },
        FeaturedSource::new("No logo", ""),
    ];
    let results = logos::sync(&projects, setup.dir.path(), &upstream()).await;
    assert_eq!(results, []);
    assert_eq!(setup.mock.requests(), []);
}

/// What finfo would name the bodies the sync sees.
#[test]
fn detects_the_image_types() {
    assert_eq!(logos::detect_mime(PNG_BYTES), "image/png");
    assert_eq!(logos::detect_mime(SVG_BYTES), "image/svg+xml");
    assert_eq!(
        logos::detect_mime(b"<?xml version=\"1.0\"?>\n<!-- c -->\n<svg viewBox=\"0 0 1 1\"/>"),
        "image/svg+xml"
    );
    assert_eq!(
        logos::detect_mime(b"\xff\xd8\xff\xe0\0\x10JFIF\0"),
        "image/jpeg"
    );
    assert_eq!(logos::detect_mime(b"404: Not Found"), "text/plain");
    assert_eq!(
        logos::detect_mime(b"<html><body>hi</body></html>"),
        "text/html"
    );
    assert_eq!(
        logos::detect_mime(b"\0\x01\x02\x03"),
        "application/octet-stream"
    );
    assert_eq!(logos::detect_mime(b""), "application/x-empty");
}
