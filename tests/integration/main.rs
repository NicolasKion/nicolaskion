//! The single integration-test binary: every suite lives here as a module
//! so a full `cargo test` links one executable.

mod common;

mod caching;
mod harness;
mod logos;
mod projects_page;
mod routes;
mod upstream_requests;
