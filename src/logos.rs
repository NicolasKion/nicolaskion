//! `sync_logos`: downloads the featured project logos from their source
//! repositories into the frontend's static directory. A download that
//! fails or is not the expected image type leaves the existing file alone.

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

/// The MIME type of a downloaded body, as `finfo` named it: PNG and other
/// binary formats by signature, SVG by its root element, else text or
/// binary.
pub fn detect_mime(_body: &[u8]) -> String {
    String::new()
}

/// Every distinct logo with a source, synced in order.
pub async fn sync(
    _projects: &[FeaturedSource],
    _static_dir: &Path,
    _upstream: &Upstream,
) -> Vec<LogoResult> {
    Vec::new()
}
