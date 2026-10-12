//! The single integration-test binary: every suite lives here as a module
//! so a full `cargo test` links one executable. Golden comparisons with
//! the Laravel reference live in their own modules (`golden_*`) with
//! their helpers in `common::golden`.

mod common;

mod caching;
mod golden_http;
mod golden_pages;
mod golden_php;
mod harness;
mod logos;
mod projects_page;
mod routes;
mod upstream_requests;
