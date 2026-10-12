//! The landing page props: the GitHub profile, the EVE pilot, the featured
//! projects, the public repositories and the stack, as the Laravel
//! `ProjectsController` built them.

use serde::Serialize;

use crate::featured::FeaturedSource;
use crate::github::Repository;

/// Languages that say nothing about what is being built.
pub const NON_APP_LANGUAGES: [&str; 4] = ["Dockerfile", "Shell", "HTML", "CSS"];

/// One `featured` entry, its GitHub fields from the matching public
/// repository (null while the source is closed).
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FeaturedProject {
    pub name: String,
    pub description: String,
    pub logo: Option<String>,
    pub logo_dark: Option<String>,
    pub site_url: Option<String>,
    pub github_url: Option<String>,
    pub language: Option<String>,
    pub stargazers_count: Option<i64>,
}

/// The repositories the page lists: public, not forks, and not dot
/// repositories like `.github`, in GitHub's order.
pub fn public_repositories(_all: &[Repository]) -> Vec<Repository> {
    Vec::new()
}

/// The three most used languages over every repository the token sees,
/// private ones included, forks and non-app languages left out. Ties keep
/// the order the languages first appear in.
pub fn stack_languages(_all: &[Repository]) -> Vec<String> {
    Vec::new()
}

/// The featured projects, each matched by exact `fullName` to a public
/// repository.
pub fn featured_projects(
    _sources: &[FeaturedSource],
    _public: &[Repository],
) -> Vec<FeaturedProject> {
    Vec::new()
}
