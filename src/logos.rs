//! `sync_logos`: downloads the featured project logos from their source
//! repositories into the frontend's static directory. A download that
//! fails or is not the expected image type leaves the existing file alone.

use std::collections::HashSet;
use std::path::Path;

use crate::featured::FeaturedSource;
use crate::upstream::Upstream;

/// The outcome for one logo path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogoResult {
    pub path: String,
    pub source: String,
    /// `None` when the logo was written.
    pub error: Option<String>,
}

/// The MIME type each logo extension must have.
fn expected_mime(path: &str) -> Option<&'static str> {
    match Path::new(path).extension()?.to_str()? {
        "png" => Some("image/png"),
        "svg" => Some("image/svg+xml"),
        _ => None,
    }
}

/// The MIME type of a downloaded body, as `finfo` named it: PNG and other
/// binary formats by signature (the `infer` crate), SVG by its root
/// element after an optional XML declaration, comments and doctype, else
/// HTML, plain text or binary.
pub fn detect_mime(body: &[u8]) -> String {
    if body.is_empty() {
        return "application/x-empty".to_owned();
    }
    // SVG first: `infer` names any XML document text/xml.
    if let Ok(text) = std::str::from_utf8(body)
        && root_element(&text.to_ascii_lowercase()).is_some_and(|root| root.starts_with("<svg"))
    {
        return "image/svg+xml".to_owned();
    }
    if let Some(kind) = infer::get(body) {
        return kind.mime_type().to_owned();
    }
    let Ok(text) = std::str::from_utf8(body) else {
        return "application/octet-stream".to_owned();
    };
    if text
        .chars()
        .any(|c| c.is_control() && !matches!(c, '\n' | '\r' | '\t' | '\u{c}'))
    {
        return "application/octet-stream".to_owned();
    }
    let lower = text.to_ascii_lowercase();
    if lower.trim_start().starts_with("<html") || lower.trim_start().starts_with("<!doctype html") {
        return "text/html".to_owned();
    }
    "text/plain".to_owned()
}

/// The text from the first element on, past an XML declaration,
/// comments and a doctype.
fn root_element(text: &str) -> Option<&str> {
    let mut rest = text.trim_start();
    loop {
        if let Some(after) = rest.strip_prefix("<?") {
            rest = after.split_once("?>")?.1.trim_start();
        } else if let Some(after) = rest.strip_prefix("<!--") {
            rest = after.split_once("-->")?.1.trim_start();
        } else if rest.starts_with("<!doctype svg") {
            rest = rest.split_once('>')?.1.trim_start();
        } else {
            return rest.starts_with('<').then_some(rest);
        }
    }
}

/// Every distinct logo with a source, synced in order.
pub async fn sync(
    projects: &[FeaturedSource],
    static_dir: &Path,
    upstream: &Upstream,
) -> Vec<LogoResult> {
    let mut seen = HashSet::new();
    let mut results = Vec::new();
    for project in projects {
        for (logo, source) in [
            (project.logo, project.logo_source),
            (project.logo_dark, project.logo_dark_source),
        ] {
            let (Some(logo), Some(source)) = (logo, source) else {
                continue;
            };
            if !seen.insert(logo) {
                continue;
            }
            results.push(LogoResult {
                path: logo.to_owned(),
                source: source.to_owned(),
                error: sync_one(logo, source, static_dir, upstream).await.err(),
            });
        }
    }
    results
}

/// Downloads one logo; a failure leaves the existing file untouched.
async fn sync_one(
    logo: &str,
    source: &str,
    static_dir: &Path,
    upstream: &Upstream,
) -> Result<(), String> {
    let (status, body) = upstream
        .get_bytes(source)
        .await
        .map_err(|error| format!("download failed: {error}"))?;
    if !(200..300).contains(&status) {
        return Err(format!("download failed with status {status}"));
    }
    let expected = expected_mime(logo).unwrap_or_default();
    let actual = detect_mime(&body);
    if actual != expected {
        return Err(format!("expected {expected} but received {actual}"));
    }
    let file = static_dir.join(logo.trim_start_matches('/'));
    tokio::fs::write(&file, body)
        .await
        .map_err(|error| format!("{}: {error}", file.display()))
}
