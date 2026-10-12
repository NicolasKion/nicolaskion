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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::featured::FeaturedSource;
    use crate::github::Owner;

    fn repository(full_name: &str, language: Option<&str>) -> Repository {
        let (owner, name) = full_name.split_once('/').expect("owner/name");
        Repository {
            id: 1,
            name: name.to_owned(),
            full_name: full_name.to_owned(),
            owner: Owner {
                login: owner.to_owned(),
                avatar_url: String::new(),
            },
            description: None,
            html_url: format!("https://github.com/{full_name}"),
            homepage: None,
            language: language.map(str::to_owned),
            stargazers_count: 4,
            forks_count: 0,
            open_issues_count: 0,
            is_private: false,
            is_fork: false,
            is_archived: false,
            pushed_at: None,
            topics: Vec::new(),
        }
    }

    #[test]
    fn private_forked_and_dot_repositories_are_not_listed() {
        let mut private = repository("a/private", None);
        private.is_private = true;
        let mut fork = repository("a/fork", None);
        fork.is_fork = true;
        let all = [
            repository("a/one", None),
            private,
            fork,
            repository("a/.github", None),
            repository("a/.dotfiles", None),
            repository("a/two.dots", None),
        ];
        let names: Vec<String> = public_repositories(&all)
            .into_iter()
            .map(|repository| repository.full_name)
            .collect();
        assert_eq!(names, ["a/one", "a/two.dots"]);
    }

    #[test]
    fn the_stack_keeps_first_seen_order_on_ties() {
        let all = [
            repository("a/1", Some("TypeScript")),
            repository("a/2", Some("Rust")),
            repository("a/3", Some("Rust")),
            repository("a/4", Some("CSS")),
            repository("a/5", Some("CSS")),
            repository("a/6", Some("Go")),
            repository("a/7", Some("TypeScript")),
            repository("a/8", Some("PHP")),
        ];
        assert_eq!(stack_languages(&all), ["TypeScript", "Rust", "Go"]);
        assert_eq!(stack_languages(&all[..1]), ["TypeScript"]);
        assert_eq!(stack_languages(&[]), Vec::<String>::new());
    }

    #[test]
    fn featured_projects_take_their_stats_from_the_public_repository() {
        let sources = [
            FeaturedSource {
                logo: Some("/logos/a.svg"),
                logo_dark: Some("/logos/a-dark.svg"),
                site_url: Some("https://a.example"),
                repository: Some("a/one"),
                ..FeaturedSource::new("One", "First")
            },
            FeaturedSource {
                repository: Some("a/missing"),
                ..FeaturedSource::new("Two", "Second")
            },
            FeaturedSource::new("Three", "Third"),
        ];
        let featured = featured_projects(&sources, &[repository("a/one", Some("Rust"))]);
        assert_eq!(
            serde_json::to_value(&featured).expect("json"),
            serde_json::json!([
                {
                    "name": "One",
                    "description": "First",
                    "logo": "/logos/a.svg",
                    "logoDark": "/logos/a-dark.svg",
                    "siteUrl": "https://a.example",
                    "githubUrl": "https://github.com/a/one",
                    "language": "Rust",
                    "stargazersCount": 4,
                },
                {
                    "name": "Two",
                    "description": "Second",
                    "logo": null,
                    "logoDark": null,
                    "siteUrl": null,
                    "githubUrl": null,
                    "language": null,
                    "stargazersCount": null,
                },
                {
                    "name": "Three",
                    "description": "Third",
                    "logo": null,
                    "logoDark": null,
                    "siteUrl": null,
                    "githubUrl": null,
                    "language": null,
                    "stargazersCount": null,
                },
            ])
        );
    }
}
