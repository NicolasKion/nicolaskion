//! The hand-picked projects highlighted on the landing page. `repository`
//! names a GitHub "owner/name" and links the source code with live stats,
//! but only while that repository is public. Logos live in
//! `frontend/static/logos` and are referenced by absolute path; each
//! `logo_source` points at the icon in the project's own repository, and
//! `cargo run --bin sync_logos` downloads those into `frontend/static`.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FeaturedSource {
    pub name: &'static str,
    pub description: &'static str,
    pub logo: Option<&'static str>,
    pub logo_source: Option<&'static str>,
    /// The logo for dark mode, when the default does not work on dark
    /// backgrounds.
    pub logo_dark: Option<&'static str>,
    pub logo_dark_source: Option<&'static str>,
    pub site_url: Option<&'static str>,
    pub repository: Option<&'static str>,
}

impl FeaturedSource {
    /// An entry with only the fields every project has; the rest are set
    /// with struct update syntax.
    pub const fn new(name: &'static str, description: &'static str) -> Self {
        Self {
            name,
            description,
            logo: None,
            logo_source: None,
            logo_dark: None,
            logo_dark_source: None,
            site_url: None,
            repository: None,
        }
    }
}

const WORMHOLE_SYSTEMS_LOGO: &str = "/logos/wormholesystems.png";
const WORMHOLE_SYSTEMS_LOGO_SOURCE: &str = "https://raw.githubusercontent.com/WormholeSystems/WormholeSystems/HEAD/public/apple-touch-icon.png";
const WORMHOLE_SYSTEMS_LOGO_DARK: &str = "/logos/wormholesystems-light.png";
const WORMHOLE_SYSTEMS_LOGO_DARK_SOURCE: &str = "https://raw.githubusercontent.com/WormholeSystems/WormholeSystems/HEAD/public/apple-touch-icon-dark.png";

pub const PROJECTS: [FeaturedSource; 8] = [
    FeaturedSource {
        logo: Some("/logos/mutamarket.svg"),
        logo_source: Some(
            "https://raw.githubusercontent.com/MutaMarket/MutaMarket/HEAD/frontend/src/lib/assets/favicon.svg",
        ),
        site_url: Some("https://mutamarket.com"),
        repository: Some("MutaMarket/MutaMarket"),
        ..FeaturedSource::new(
            "MutaMarket",
            "The marketplace for all your abyssal module needs in EVE Online. Trade, appraise, and browse mutated modules.",
        )
    },
    FeaturedSource {
        logo: Some(WORMHOLE_SYSTEMS_LOGO),
        logo_source: Some(WORMHOLE_SYSTEMS_LOGO_SOURCE),
        logo_dark: Some(WORMHOLE_SYSTEMS_LOGO_DARK),
        logo_dark_source: Some(WORMHOLE_SYSTEMS_LOGO_DARK_SOURCE),
        site_url: Some("https://wormhole.systems"),
        repository: Some("WormholeSystems/WormholeSystems"),
        ..FeaturedSource::new(
            "wormhole.systems",
            "A collaborative wormhole mapping and tracking tool for EVE Online.",
        )
    },
    FeaturedSource {
        logo: Some("/logos/evewrench.png"),
        logo_source: Some(
            "https://raw.githubusercontent.com/eve-wrench/eve-wrench-app/HEAD/assets/icons/icon.png",
        ),
        repository: Some("eve-wrench/eve-wrench-app"),
        ..FeaturedSource::new(
            "EVE Wrench",
            "Backup, restore, and sync your EVE Online settings across accounts and characters.",
        )
    },
    FeaturedSource {
        logo: Some("/logos/loganalyzer.svg"),
        logo_source: Some(
            "https://raw.githubusercontent.com/eve-telescope/loganalyzer/HEAD/public/favicon.svg",
        ),
        site_url: Some("https://loganalyzer.eve-telescope.com"),
        repository: Some("eve-telescope/loganalyzer"),
        ..FeaturedSource::new(
            "Log Analyzer",
            "Analyze your EVE Online combat logs, part of the Telescope toolkit.",
        )
    },
    FeaturedSource {
        logo: Some("/logos/killfeed.svg"),
        logo_source: Some(
            "https://raw.githubusercontent.com/NicolasKion/KillFeed/HEAD/public/favicon.svg",
        ),
        site_url: Some("https://killfeed.killmail.space"),
        repository: Some("NicolasKion/KillFeed"),
        ..FeaturedSource::new(
            "Killfeed",
            "A real-time killmail feed for EVE Online at killfeed.killmail.space.",
        )
    },
    FeaturedSource {
        logo: Some("/logos/telescope.png"),
        logo_source: Some(
            "https://raw.githubusercontent.com/eve-telescope/telescope-app/HEAD/crates/telescope-app/icons/icon.png",
        ),
        site_url: Some("https://eve-telescope.com"),
        repository: Some("eve-telescope/telescope-app"),
        ..FeaturedSource::new("Telescope", "A local scanner tool for EVE Online.")
    },
    FeaturedSource {
        logo: Some("/logos/marketeer.png"),
        logo_source: Some(
            "https://raw.githubusercontent.com/eve-marketeer/marketeer/HEAD/src-tauri/icons/icon.png",
        ),
        repository: Some("eve-marketeer/marketeer"),
        ..FeaturedSource::new(
            "Marketeer",
            "A desktop market tool for EVE Online that monitors your market exports, calculates competitive prices, and copies them to your clipboard.",
        )
    },
    FeaturedSource {
        logo: Some(WORMHOLE_SYSTEMS_LOGO),
        logo_source: Some(WORMHOLE_SYSTEMS_LOGO_SOURCE),
        logo_dark: Some(WORMHOLE_SYSTEMS_LOGO_DARK),
        logo_dark_source: Some(WORMHOLE_SYSTEMS_LOGO_DARK_SOURCE),
        repository: Some("WormholeSystems/wormholesystems-containers"),
        ..FeaturedSource::new(
            "wormhole.systems Containers",
            "A Docker container setup to self-host a private wormhole.systems instance.",
        )
    },
];
