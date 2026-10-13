//! The landing page props against the mock GitHub and ESI, mostly with
//! the two featured projects of `test_featured`.

use axum::http::StatusCode;
use serde_json::{Value, json};

use nicolaskion::featured::{FeaturedSource, PROJECTS};

use crate::common::{
    CHARACTER_ID, Service, TestApp, assert_json_eq, key_paths, repo_path, test_featured,
};

/// A GitHub repository answer with overrides.
fn github_repository(overrides: Value) -> Value {
    let mut repository = json!({
        "id": 1,
        "name": "killfeed",
        "full_name": "WormholeSystems/killfeed",
        "owner": {
            "login": "WormholeSystems",
            "avatar_url": "https://avatars.githubusercontent.com/u/1?v=4",
        },
        "description": "A killmail feed",
        "html_url": "https://github.com/WormholeSystems/killfeed",
        "homepage": null,
        "language": "Vue",
        "stargazers_count": 5,
        "forks_count": 0,
        "open_issues_count": 0,
        "private": false,
        "fork": false,
        "archived": false,
        "pushed_at": "2026-07-01T00:00:00Z",
        "topics": [],
    });
    for (key, value) in overrides.as_object().expect("overrides") {
        repository[key] = value.clone();
    }
    repository
}

fn profile() -> Value {
    json!({
        "login": "NicolasKion",
        "name": "Nicolas Kion",
        "avatar_url": "https://avatars.githubusercontent.com/u/172045106?v=4",
        "html_url": "https://github.com/NicolasKion",
    })
}

/// GitHub configured with `repositories`, ESI not configured, the suite's
/// featured projects.
async fn app_with_repositories(repositories: Vec<Value>) -> TestApp {
    let app = TestApp::with(|config| {
        config.featured = test_featured();
        config.esi.character_id = None;
    })
    .await;
    app.upstream.change(|data| {
        data.user = profile();
        data.repositories = repositories;
    });
    app
}

/// Every prop with its exact keys, in order, and the values mapped from
/// the GitHub and ESI answers.
#[tokio::test]
async fn the_props_have_their_exact_shape_and_key_order() {
    let app = TestApp::with(|config| {
        config.featured = vec![
            FeaturedSource {
                logo: Some("/logos/wormholesystems.png"),
                logo_dark: Some("/logos/wormholesystems-light.png"),
                site_url: Some("https://wormhole.systems"),
                repository: Some("WormholeSystems/killfeed"),
                ..FeaturedSource::new("wormhole.systems", "Wormhole mapping.")
            },
            FeaturedSource::new("EVE Wrench", "Settings sync."),
        ];
    })
    .await;
    app.upstream.change(|data| {
        data.user = profile();
        data.repositories = vec![
            github_repository(json!({
                "homepage": "https://killfeed.example",
                "topics": ["eve-online", "rust"],
                "stargazers_count": 40,
                "forks_count": 9,
                "open_issues_count": 12,
            })),
            github_repository(json!({
                "id": 2,
                "name": "old",
                "full_name": "NicolasKion/old",
                "owner": { "login": "NicolasKion", "avatar_url": "https://avatars.example/2" },
                "description": null,
                "html_url": "https://github.com/NicolasKion/old",
                "homepage": "",
                "language": null,
                "archived": true,
                "pushed_at": "2021-09-12T07:30:00Z",
            })),
        ];
    });

    let expected = json!({
        "name": "Nicolas Kion",
        "auth": { "user": null },
        "githubConfigured": true,
        "profile": {
            "login": "NicolasKion",
            "name": "Nicolas Kion",
            "avatarUrl": "https://avatars.githubusercontent.com/u/172045106?v=4",
            "htmlUrl": "https://github.com/NicolasKion",
        },
        "character": {
            "id": 95569887,
            "name": "Nicolas Kion",
            "corporationName": "Strix Ridens",
            "corporationTicker": "OWL.",
            "allianceName": "Strix Ridens.",
            "allianceTicker": "STRIX",
            "securityStatus": -0.5,
            "pilotSince": 2015,
            "portraitUrl": "https://images.evetech.net/characters/95569887/portrait?size=128",
            "zkillboardUrl": "https://zkillboard.com/character/95569887/",
            "eveWhoUrl": "https://evewho.com/character/95569887",
        },
        "featured": [
            {
                "name": "wormhole.systems",
                "description": "Wormhole mapping.",
                "logo": "/logos/wormholesystems.png",
                "logoDark": "/logos/wormholesystems-light.png",
                "siteUrl": "https://wormhole.systems",
                "githubUrl": "https://github.com/WormholeSystems/killfeed",
                "language": "Vue",
                "stargazersCount": 40,
            },
            {
                "name": "EVE Wrench",
                "description": "Settings sync.",
                "logo": null,
                "logoDark": null,
                "siteUrl": null,
                "githubUrl": null,
                "language": null,
                "stargazersCount": null,
            },
        ],
        "repositories": [
            {
                "id": 1,
                "name": "killfeed",
                "fullName": "WormholeSystems/killfeed",
                "owner": {
                    "login": "WormholeSystems",
                    "avatarUrl": "https://avatars.githubusercontent.com/u/1?v=4",
                },
                "description": "A killmail feed",
                "htmlUrl": "https://github.com/WormholeSystems/killfeed",
                "homepage": "https://killfeed.example",
                "language": "Vue",
                "stargazersCount": 40,
                "forksCount": 9,
                "openIssuesCount": 12,
                "isPrivate": false,
                "isFork": false,
                "isArchived": false,
                "pushedAt": "2026-07-01T00:00:00Z",
                "topics": ["eve-online", "rust"],
            },
            {
                "id": 2,
                "name": "old",
                "fullName": "NicolasKion/old",
                "owner": { "login": "NicolasKion", "avatarUrl": "https://avatars.example/2" },
                "description": null,
                "htmlUrl": "https://github.com/NicolasKion/old",
                "homepage": "",
                "language": null,
                "stargazersCount": 5,
                "forksCount": 0,
                "openIssuesCount": 0,
                "isPrivate": false,
                "isFork": false,
                "isArchived": true,
                "pushedAt": "2021-09-12T07:30:00Z",
                "topics": [],
            },
        ],
        "stack": ["Vue"],
    });
    let actual = app.home().await;
    assert_json_eq(&expected, &actual);
    assert_eq!(key_paths(&actual), key_paths(&expected));
}

/// The page's props load.
#[tokio::test]
async fn returns_a_successful_response() {
    let app = TestApp::new().await;
    assert_eq!(app.get("/api/pages/home").await.status, StatusCode::OK);
}

#[tokio::test]
async fn renders_the_landing_page_with_public_repositories_and_featured_projects() {
    let app = app_with_repositories(vec![
        github_repository(json!({})),
        github_repository(json!({
            "id": 2, "name": "secret", "full_name": "NicolasKion/secret", "private": true,
        })),
        github_repository(json!({
            "id": 3, "name": "some-fork", "full_name": "NicolasKion/some-fork", "fork": true,
        })),
        github_repository(json!({
            "id": 4, "name": ".github", "full_name": "WormholeSystems/.github",
        })),
    ])
    .await;

    let page = app.home().await;
    assert_eq!(page["profile"]["login"], "NicolasKion");
    assert_eq!(
        page["repositories"].as_array().expect("repositories").len(),
        1
    );
    assert_eq!(
        page["repositories"][0]["fullName"],
        "WormholeSystems/killfeed"
    );
    assert_eq!(page["featured"].as_array().expect("featured").len(), 2);
    assert_eq!(page["featured"][0]["name"], "MutaMarket");
    assert_eq!(page["featured"][0]["githubUrl"], Value::Null);
    assert_eq!(
        page["featured"][1]["githubUrl"],
        "https://github.com/WormholeSystems/killfeed"
    );
    assert_eq!(page["featured"][1]["stargazersCount"], 5);
    assert_eq!(page["stack"], json!(["Vue"]));
}

#[tokio::test]
async fn does_not_link_private_repositories_in_featured_projects() {
    let app = app_with_repositories(vec![github_repository(json!({ "private": true }))]).await;

    let page = app.home().await;
    assert_eq!(page["repositories"], json!([]));
    assert_eq!(page["featured"][1]["githubUrl"], Value::Null);
    assert_eq!(page["featured"][1]["language"], Value::Null);
    assert_eq!(page["featured"][1]["stargazersCount"], Value::Null);
}

#[tokio::test]
async fn renders_without_github_data_when_no_token_is_configured() {
    for token in [None, Some(""), Some("  \t")] {
        let app = TestApp::with(|config| {
            config.featured = test_featured();
            config.github.token = token.map(str::to_owned);
            config.esi.character_id = None;
        })
        .await;

        let page = app.home().await;
        assert_eq!(page["profile"], Value::Null, "{token:?}");
        assert_eq!(page["repositories"], json!([]), "{token:?}");
        assert_eq!(page["stack"], json!([]), "{token:?}");
        assert_eq!(page["featured"].as_array().expect("featured").len(), 2);
        assert_eq!(page["githubConfigured"], json!(false));
        assert_eq!(app.upstream.requests(), [], "{token:?}");
    }
}

#[tokio::test]
async fn shows_the_eve_character_in_the_dossier() {
    let app = TestApp::with(|config| config.featured = test_featured()).await;
    app.upstream.change(|data| {
        data.user = profile();
        data.repositories = vec![github_repository(json!({}))];
        data.character = json!({
            "name": "Nicolas Kion",
            "corporation_id": 98630705,
            "alliance_id": 99014466,
            "security_status": -0.5,
            "birthday": "2015-05-05T14:16:51Z",
        });
        data.corporation = json!({ "name": "Strix Ridens", "ticker": "OWL." });
        data.alliance = json!({ "name": "Strix Ridens.", "ticker": "STRIX" });
    });

    let character = app.home().await["character"].clone();
    assert_eq!(character["name"], "Nicolas Kion");
    assert_eq!(character["corporationTicker"], "OWL.");
    assert_eq!(character["allianceName"], "Strix Ridens.");
    assert_eq!(character["securityStatus"], json!(-0.5));
    assert_eq!(character["pilotSince"], 2015);
    assert_eq!(
        character["zkillboardUrl"],
        format!("https://zkillboard.com/character/{CHARACTER_ID}/")
    );
}

#[tokio::test]
async fn renders_without_a_character_when_esi_is_not_configured() {
    for character_id in [None, Some(""), Some("   ")] {
        let app = TestApp::with(|config| {
            config.featured = test_featured();
            config.esi.character_id = character_id.map(str::to_owned);
        })
        .await;
        assert_eq!(app.home().await["character"], Value::Null);
        assert_eq!(app.upstream.requests_to(Service::Esi), []);
    }
}

#[tokio::test]
async fn fetches_repositories_across_multiple_pages() {
    let repositories = (1..=101)
        .map(|index| {
            github_repository(json!({
                "id": index,
                "full_name": format!("NicolasKion/repo-{index}"),
            }))
        })
        .collect();
    let app = app_with_repositories(repositories).await;

    let page = app.home().await;
    assert_eq!(
        page["repositories"].as_array().expect("repositories").len(),
        101
    );
    let pages: Vec<String> = app
        .upstream
        .requests_to(Service::GitHub)
        .into_iter()
        .filter(|request| request.path == "/github/user/repos")
        .map(|request| request.query["page"].clone())
        .collect();
    assert_eq!(pages, ["1", "2"]);
}

#[test]
fn references_existing_logos_with_sources_and_well_formed_repositories_in_the_featured_config() {
    let well_formed = |repository: &str| {
        let mut parts = repository.split('/');
        let valid = |part: &str| {
            !part.is_empty()
                && part
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '.' | '-'))
        };
        matches!((parts.next(), parts.next(), parts.next()), (Some(owner), Some(name), None) if valid(owner) && valid(name))
    };
    for project in PROJECTS {
        for (logo, source) in [
            (project.logo, project.logo_source),
            (project.logo_dark, project.logo_dark_source),
        ] {
            let Some(logo) = logo else { continue };
            let file = repo_path(&format!("frontend/static{logo}"));
            assert!(file.is_file(), "{} logo {logo} is missing", project.name);
            assert!(
                source.is_some_and(|source| source.starts_with("https://")),
                "{} {logo} has no source to sync from",
                project.name
            );
        }
        if let Some(repository) = project.repository {
            assert!(well_formed(repository), "{repository}");
        }
    }
}

#[tokio::test]
async fn links_the_mutamarket_source_once_the_repository_is_public() {
    let app = TestApp::with(|config| {
        config.featured = vec![FeaturedSource {
            logo: Some("/logos/mutamarket.svg"),
            site_url: Some("https://mutamarket.com"),
            repository: Some("MutaMarket/MutaMarket"),
            ..FeaturedSource::new("MutaMarket", "Open source marketplace.")
        }];
        config.esi.character_id = None;
    })
    .await;
    app.upstream.change(|data| {
        data.user = profile();
        data.repositories = vec![github_repository(json!({
            "name": "MutaMarket",
            "full_name": "MutaMarket/MutaMarket",
            "html_url": "https://github.com/MutaMarket/MutaMarket",
            "language": "Rust",
            "stargazers_count": 3,
        }))];
    });

    let featured = app.home().await["featured"][0].clone();
    assert_eq!(featured["name"], "MutaMarket");
    assert_eq!(
        featured["githubUrl"],
        "https://github.com/MutaMarket/MutaMarket"
    );
    assert_eq!(featured["language"], "Rust");
    assert_eq!(featured["stargazersCount"], 3);
}

/// The featured lookup is by exact, case-sensitive full name.
#[tokio::test]
async fn featured_repositories_match_their_full_name_exactly() {
    let app = app_with_repositories(vec![github_repository(json!({
        "full_name": "wormholesystems/Killfeed",
    }))])
    .await;
    assert_eq!(app.home().await["featured"][1]["githubUrl"], Value::Null);
}

/// A GitHub failure with nothing cached renders the page without GitHub
/// and with the dossier from ESI.
#[tokio::test]
async fn a_github_failure_with_nothing_cached_renders_without_github() {
    let app = TestApp::with(|config| config.featured = test_featured()).await;
    app.upstream.fail_with(Service::GitHub, 500);

    let response = app.get("/api/pages/home").await;
    assert_eq!(response.status, StatusCode::OK);
    let page = response.json();
    assert_eq!(page["githubConfigured"], json!(true));
    assert_eq!(page["profile"], Value::Null);
    assert_eq!(page["repositories"], json!([]));
    assert_eq!(page["stack"], json!([]));
    assert_eq!(page["featured"][1]["githubUrl"], Value::Null);
    assert_eq!(page["character"]["name"], "Nicolas Kion");
}

/// The profile and the repositories fail independently.
#[tokio::test]
async fn a_failing_profile_keeps_the_repositories() {
    let app = TestApp::new().await;
    app.upstream
        .change(|data| data.user = json!({ "unexpected": true }));

    let page = app.home().await;
    assert_eq!(page["profile"], Value::Null);
    assert_eq!(
        page["repositories"].as_array().expect("repositories").len(),
        12
    );
}

/// An ESI failure leaves the dossier out.
#[tokio::test]
async fn an_esi_failure_renders_without_a_character() {
    for status in [404, 420, 500, 503] {
        let app = TestApp::new().await;
        app.upstream.fail_with(Service::Esi, status);
        let page = app.home().await;
        assert_eq!(page["character"], Value::Null, "{status}");
        assert_eq!(page["profile"]["login"], "NicolasKion", "{status}");
    }
}

/// A character without an alliance or a security status: no alliance
/// request, null alliance fields, a security status of 0 (written as an
/// integer).
#[tokio::test]
async fn a_character_without_alliance_or_security_status() {
    let app = TestApp::new().await;
    app.upstream.scenario("no-alliance");

    let character = app.home().await["character"].clone();
    assert_eq!(character["allianceName"], Value::Null);
    assert_eq!(character["allianceTicker"], Value::Null);
    assert_eq!(character["securityStatus"].to_string(), "0");
    assert_eq!(character["pilotSince"], 2008);
    let paths: Vec<String> = app
        .upstream
        .requests_to(Service::Esi)
        .into_iter()
        .map(|request| request.path)
        .collect();
    assert_eq!(
        paths,
        ["/esi/characters/95569887", "/esi/corporations/98630705"]
    );
}

/// The dossier links and the portrait.
#[tokio::test]
async fn the_character_links_to_its_profiles() {
    let app = TestApp::new().await;
    let character = app.home().await["character"].clone();
    assert_eq!(character["id"], 95569887);
    assert_eq!(
        character["portraitUrl"],
        "https://images.evetech.net/characters/95569887/portrait?size=128"
    );
    assert_eq!(
        character["eveWhoUrl"],
        "https://evewho.com/character/95569887"
    );
    assert_eq!(character["securityStatus"], json!(-0.5));
}

/// The stack counts every repository the token sees except forks,
/// private ones included, skips languages that say nothing about the
/// product, and keeps first-seen order on ties.
#[tokio::test]
async fn the_stack_is_the_three_most_used_languages() {
    let repository = |id: i64, language: Option<&str>, private: bool, fork: bool| {
        github_repository(json!({
            "id": id,
            "full_name": format!("NicolasKion/repo-{id}"),
            "language": language,
            "private": private,
            "fork": fork,
        }))
    };
    let app = app_with_repositories(vec![
        repository(1, Some("Shell"), false, false),
        repository(2, Some("Go"), false, false),
        repository(3, Some("PHP"), true, false),
        repository(4, Some("Python"), false, true),
        repository(5, Some("Python"), false, true),
        repository(6, None, false, false),
        repository(7, Some("PHP"), false, false),
        repository(8, Some("Rust"), false, false),
        repository(9, Some("Shell"), false, false),
        repository(10, Some("Dockerfile"), false, false),
        repository(11, Some("HTML"), false, false),
        repository(12, Some("CSS"), false, false),
        repository(13, Some("Go"), true, false),
        repository(14, Some("Rust"), false, false),
        repository(15, Some("Vue"), false, false),
    ])
    .await;

    assert_eq!(app.home().await["stack"], json!(["Go", "PHP", "Rust"]));
}

/// The shared props: the app name and no user.
#[tokio::test]
async fn the_shared_props_carry_the_app_name() {
    let app = TestApp::with(|config| config.app_name = "Someone Else".to_owned()).await;
    let page = app.home().await;
    assert_eq!(page["name"], "Someone Else");
    assert_eq!(page["auth"], json!({ "user": null }));
}
