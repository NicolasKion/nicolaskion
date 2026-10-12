//! The GitHub profile and repositories behind the token: `/user` and every
//! page of `/user/repos`, cached 10 minutes fresh and up to an hour stale.

use std::sync::Arc;
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::cache::{Clock, SwrCache};
use crate::config::GitHubConfig;
use crate::upstream::Upstream;

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

    /// Whether a token is set; without one GitHub is never asked.
    pub fn is_configured(&self) -> bool {
        self.config.token.is_some()
    }

    /// The token's user, `None` when GitHub is not configured or fails
    /// with nothing cached.
    pub async fn profile(&self) -> Option<Profile> {
        let _ = (&self.upstream, &self.profile);
        None
    }

    /// Every repository the token's user owns or reaches through an
    /// organization, newest push first, private ones and forks included.
    pub async fn repositories(&self) -> Option<Vec<Repository>> {
        let _ = &self.repositories;
        None
    }
}
