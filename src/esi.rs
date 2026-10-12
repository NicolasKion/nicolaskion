//! The EVE Online pilot in the dossier: the character, its corporation and
//! its alliance from ESI's public routes, cached an hour fresh and up to a
//! day stale.

use std::sync::Arc;
use std::time::Duration;

use serde::{Deserialize, Serialize, Serializer};

use crate::cache::{Clock, SwrCache};
use crate::config::EsiConfig;
use crate::upstream::Upstream;

pub const FRESH: Duration = Duration::from_secs(3600);
pub const STALE: Duration = Duration::from_secs(86400);
/// The ESI behavior this client is written against (`X-Compatibility-Date`).
pub const COMPATIBILITY_DATE: &str = "2026-08-18";

/// What ESI says about the character, its corporation and alliance.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct CharacterData {
    pub character: CharacterInfo,
    pub corporation: Named,
    pub alliance: Option<Named>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct CharacterInfo {
    pub name: String,
    pub corporation_id: i64,
    pub alliance_id: Option<i64>,
    pub security_status: Option<f64>,
    pub birthday: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Named {
    pub name: String,
    pub ticker: String,
}

/// The `character` prop.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EveCharacter {
    pub id: i64,
    pub name: String,
    pub corporation_name: String,
    pub corporation_ticker: String,
    pub alliance_name: Option<String>,
    pub alliance_ticker: Option<String>,
    #[serde(serialize_with = "php_float")]
    pub security_status: f64,
    pub pilot_since: i32,
    pub portrait_url: String,
    pub zkillboard_url: String,
    pub eve_who_url: String,
}

/// PHP's `json_encode` writes a float without a fraction as an integer
/// (`0`, `-5`); the props keep that shape.
fn php_float<S: Serializer>(value: &f64, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_f64(*value)
}

/// PHP's `round($value, 1)`: half away from zero, on the decimal value
/// PHP prints (15 significant digits), so 0.35 rounds to 0.4.
pub fn round_one_decimal(value: f64) -> f64 {
    value
}

/// The year of an ESI birthday (`2015-05-05T14:16:51Z`), in UTC.
pub fn birth_year(_birthday: &str) -> Option<i32> {
    None
}

/// PHP's `(int)` cast of the configured id: the leading integer, else 0.
pub fn character_id(_configured: &str) -> i64 {
    0
}

impl EveCharacter {
    pub fn from_data(_id: i64, _data: &CharacterData) -> Option<Self> {
        None
    }
}

#[derive(Clone)]
pub struct Esi {
    config: EsiConfig,
    upstream: Upstream,
    cache: Arc<SwrCache<CharacterData>>,
}

impl Esi {
    pub fn new(config: EsiConfig, upstream: Upstream, clock: Clock) -> Self {
        Self {
            config,
            upstream,
            cache: SwrCache::new("esi.character", FRESH, STALE, clock),
        }
    }

    pub fn is_configured(&self) -> bool {
        self.config.character_id.is_some()
    }

    /// The pilot, `None` when ESI is not configured or fails with nothing
    /// cached: the page renders without the dossier then.
    pub async fn character(&self) -> Option<EveCharacter> {
        let _ = (&self.upstream, &self.cache);
        None
    }
}
