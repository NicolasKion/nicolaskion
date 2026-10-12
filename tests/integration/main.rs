//! The single integration-test binary: every suite lives here as a module
//! so a full `cargo test` links one executable. Golden comparisons with
//! the Laravel reference live in their own modules (`golden_*`) with
//! their helpers in `common::golden`.

mod common;

mod harness;
