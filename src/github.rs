//! The GitHub profile and repositories behind the token: `/user` and every
//! page of `/user/repos`, cached 10 minutes fresh and up to an hour stale.

use std::sync::Arc;
use std::time::Duration;

use reqwest::header::{ACCEPT, AUTHORIZATION, HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

use crate::cache::{Clock, SwrCache};
use crate::config::GitHubConfig;
use crate::upstream::{Upstream, UpstreamError};

pub const FRESH: Duration = Duration::from_secs(600);
pub const STALE: Duration = Duration::from_secs(3600);
/// GitHub's largest page; a shorter page is the last one.
pub const PER_PAGE: usize = 100;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct Profile {
    pub login: String,
    pub name: Option<String>,
    pub avatar_url: String,
    pub html_url: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct Owner {
    pub login: String,
    pub avatar_url: String,
}

/// One repository, serialized in the order the page props list it.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct Repository {
    pub id: i64,
    pub name: String,
    pub full_name: String,
    pub owner: Owner,
    pub description: Option<String>,
    pub html_url: String,
    pub homepage: Option<String>,
    pub language: Option<String>,
    pub stargazers_count: i64,
    pub forks_count: i64,
    pub open_issues_count: i64,
    #[serde(rename(deserialize = "private"))]
    pub is_private: bool,
    #[serde(rename(deserialize = "fork"))]
    pub is_fork: bool,
    #[serde(rename(deserialize = "archived"))]
    pub is_archived: bool,
    pub pushed_at: Option<String>,
    #[serde(default)]
    pub topics: Vec<String>,
}

#[derive(Clone)]
pub struct GitHub {
    config: GitHubConfig,
    upstream: Upstream,
    profile: Arc<SwrCache<Profile>>,
    repositories: Arc<SwrCache<Vec<Repository>>>,
}

impl GitHub {
    pub fn new(config: GitHubConfig, upstream: Upstream, clock: Clock) -> Self {
        Self {
            config,
            upstream,
            profile: SwrCache::new("github.profile", FRESH, STALE, clock.clone()),
            repositories: SwrCache::new("github.repositories", FRESH, STALE, clock),
        }
    }

    /// Whether a token is set (and not blank); without one GitHub is
    /// never asked.
    pub fn is_configured(&self) -> bool {
        self.token().is_some()
    }

    fn token(&self) -> Option<&str> {
        self.config
            .token
            .as_deref()
            .map(str::trim)
            .filter(|token| !token.is_empty())
    }

    /// The token's user, `None` when GitHub is not configured or fails
    /// with nothing cached.
    pub async fn profile(&self) -> Option<Profile> {
        if !self.is_configured() {
            return None;
        }
        let github = self.clone();
        self.profile
            .get(move || {
                let github = github.clone();
                async move { github.fetch_profile().await }
            })
            .await
    }

    /// Every repository the token's user owns or reaches through an
    /// organization, newest push first, private ones and forks included.
    pub async fn repositories(&self) -> Option<Vec<Repository>> {
        if !self.is_configured() {
            return None;
        }
        let github = self.clone();
        self.repositories
            .get(move || {
                let github = github.clone();
                async move { github.fetch_repositories().await }
            })
            .await
    }

    fn headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            ACCEPT,
            HeaderValue::from_static("application/vnd.github+json"),
        );
        if let Some(token) = self.token()
            && let Ok(value) = HeaderValue::from_str(&format!("Bearer {token}"))
        {
            headers.insert(AUTHORIZATION, value);
        }
        headers
    }

    async fn fetch_profile(&self) -> Result<Profile, UpstreamError> {
        let url = format!("{}/user", self.config.api_url);
        self.upstream.get_json(&url, &[], self.headers()).await
    }

    /// Page after page until one comes back shorter than `PER_PAGE`.
    async fn fetch_repositories(&self) -> Result<Vec<Repository>, UpstreamError> {
        let url = format!("{}/user/repos", self.config.api_url);
        let per_page = PER_PAGE.to_string();
        let mut repositories = Vec::new();
        for page in 1.. {
            let page = page.to_string();
            let results: Vec<Repository> = self
                .upstream
                .get_json(
                    &url,
                    &[
                        ("affiliation", "owner,organization_member"),
                        ("sort", "pushed"),
                        ("per_page", &per_page),
                        ("page", &page),
                    ],
                    self.headers(),
                )
                .await?;
            let last = results.len() < PER_PAGE;
            repositories.extend(results);
            if last {
                break;
            }
        }
        Ok(repositories)
    }
}
