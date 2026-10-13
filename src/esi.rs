//! The EVE Online pilot in the dossier: the character, its corporation and
//! its alliance from ESI's public routes, cached an hour fresh and up to a
//! day stale.

use std::sync::Arc;
use std::time::Duration;

use reqwest::header::{ACCEPT, HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize, Serializer};
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;

use crate::cache::{Clock, SwrCache};
use crate::config::EsiConfig;
use crate::upstream::{Upstream, UpstreamError};

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
    #[serde(serialize_with = "whole_as_integer")]
    pub security_status: f64,
    pub pilot_since: i32,
    pub portrait_url: String,
    pub zkillboard_url: String,
    pub eve_who_url: String,
}

/// A float without a fraction is written as an integer (`0`, `-5`).
fn whole_as_integer<S: Serializer>(value: &f64, serializer: S) -> Result<S::Ok, S::Error> {
    // Whole values within the exactly representable integers.
    const LIMIT: f64 = 9_007_199_254_740_992.0;
    if value.fract() == 0.0 && value.abs() < LIMIT {
        serializer.serialize_i64(*value as i64)
    } else {
        serializer.serialize_f64(*value)
    }
}

/// Rounds to one decimal, half away from zero, on the decimal value with
/// 15 significant digits, so 0.35 rounds to 0.4 although the double is a
/// hair below it.
pub fn round_one_decimal(value: f64) -> f64 {
    if !value.is_finite() || value == 0.0 {
        return value;
    }
    // 15 significant digits: d.dddddddddddddde<exponent>.
    let printed = format!("{:.14e}", value.abs());
    let (mantissa, exponent) = printed.split_once('e').expect("exponent form");
    let digits: u64 = mantissa.replace('.', "").parse().expect("digits");
    let exponent: i32 = exponent.parse().expect("exponent");
    // value * 10 = digits * 10^(exponent - 13)
    let shift = exponent - 13;
    let tenths = if shift >= 0 {
        digits.saturating_mul(10u64.saturating_pow(shift as u32))
    } else if shift < -18 {
        0
    } else {
        let divisor = 10u64.pow((-shift) as u32);
        let (quotient, remainder) = (digits / divisor, digits % divisor);
        if remainder * 2 >= divisor {
            quotient + 1
        } else {
            quotient
        }
    };
    let rounded = tenths as f64 / 10.0;
    if value < 0.0 { -rounded } else { rounded }
}

/// The year of an ESI birthday (`2015-05-05T14:16:51Z`) in its own
/// offset (ESI always answers in UTC).
pub fn birth_year(birthday: &str) -> Option<i32> {
    OffsetDateTime::parse(birthday, &Rfc3339)
        .ok()
        .map(OffsetDateTime::year)
}

/// The configured id: its leading integer, else 0.
pub fn character_id(configured: &str) -> i64 {
    let text = configured.trim();
    let (sign, digits) = match text.strip_prefix('-') {
        Some(rest) => (-1, rest),
        None => (1, text.strip_prefix('+').unwrap_or(text)),
    };
    let end = digits
        .find(|c: char| !c.is_ascii_digit())
        .unwrap_or(digits.len());
    digits[..end].parse::<i64>().map_or(0, |id| sign * id)
}

impl EveCharacter {
    /// The prop, `None` when the birthday does not parse.
    pub fn from_data(id: i64, data: &CharacterData) -> Option<Self> {
        let pilot_since = birth_year(&data.character.birthday)?;
        Some(Self {
            id,
            name: data.character.name.clone(),
            corporation_name: data.corporation.name.clone(),
            corporation_ticker: data.corporation.ticker.clone(),
            alliance_name: data.alliance.as_ref().map(|alliance| alliance.name.clone()),
            alliance_ticker: data
                .alliance
                .as_ref()
                .map(|alliance| alliance.ticker.clone()),
            security_status: round_one_decimal(data.character.security_status.unwrap_or(0.0)),
            pilot_since,
            portrait_url: format!("https://images.evetech.net/characters/{id}/portrait?size=128"),
            zkillboard_url: format!("https://zkillboard.com/character/{id}/"),
            eve_who_url: format!("https://evewho.com/character/{id}"),
        })
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
        self.configured_id().is_some()
    }

    fn configured_id(&self) -> Option<i64> {
        self.config
            .character_id
            .as_deref()
            .map(str::trim)
            .filter(|id| !id.is_empty())
            .map(character_id)
    }

    /// The pilot, `None` when ESI is not configured or fails with nothing
    /// cached: the page renders without the dossier then.
    pub async fn character(&self) -> Option<EveCharacter> {
        let id = self.configured_id()?;
        let esi = self.clone();
        let data = self
            .cache
            .get(move || {
                let esi = esi.clone();
                async move { esi.fetch(id).await }
            })
            .await?;
        let character = EveCharacter::from_data(id, &data);
        if character.is_none() {
            tracing::warn!(birthday = %data.character.birthday, "ESI birthday does not parse");
        }
        character
    }

    async fn get<T: serde::de::DeserializeOwned>(&self, route: &str) -> Result<T, UpstreamError> {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        headers.insert(
            "x-compatibility-date",
            HeaderValue::from_static(COMPATIBILITY_DATE),
        );
        let url = format!("{}{route}", self.config.base_url);
        self.upstream.get_json(&url, &[], headers).await
    }

    /// The character, then its corporation and its alliance, if any.
    async fn fetch(&self, id: i64) -> Result<CharacterData, UpstreamError> {
        let character: CharacterInfo = self.get(&format!("/characters/{id}")).await?;
        let corporation: Named = self
            .get(&format!("/corporations/{}", character.corporation_id))
            .await?;
        let alliance = match character.alliance_id {
            Some(alliance_id) => Some(self.get(&format!("/alliances/{alliance_id}")).await?),
            None => None,
        };
        Ok(CharacterData {
            character,
            corporation,
            alliance,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn data(security_status: Option<f64>, alliance: bool) -> CharacterData {
        CharacterData {
            character: CharacterInfo {
                name: "Nicolas Kion".to_owned(),
                corporation_id: 98630705,
                alliance_id: alliance.then_some(99014466),
                security_status,
                birthday: "2015-05-05T14:16:51Z".to_owned(),
            },
            corporation: Named {
                name: "Strix Ridens".to_owned(),
                ticker: "OWL.".to_owned(),
            },
            alliance: alliance.then(|| Named {
                name: "Strix Ridens.".to_owned(),
                ticker: "STRIX".to_owned(),
            }),
        }
    }

    #[test]
    fn the_birth_year_is_read_in_its_own_offset() {
        assert_eq!(birth_year("2015-05-05T14:16:51Z"), Some(2015));
        assert_eq!(birth_year("2008-12-31T23:59:59Z"), Some(2008));
        assert_eq!(birth_year("2008-12-31T23:30:00-01:00"), Some(2008));
        assert_eq!(birth_year("2003-05-06T12:00:00.123Z"), Some(2003));
        assert_eq!(birth_year("not a date"), None);
    }

    #[test]
    fn the_character_id_is_its_leading_integer() {
        assert_eq!(character_id("95569887"), 95569887);
        assert_eq!(character_id(" 95569887 "), 95569887);
        assert_eq!(character_id("95569887abc"), 95569887);
        assert_eq!(character_id("abc"), 0);
        assert_eq!(character_id("-5"), -5);
    }

    /// Half away from zero on the value with 15 significant digits, and
    /// the sign of a negative zero survives.
    #[test]
    fn rounding_is_half_away_from_zero() {
        let cases = [
            (-0.45, -0.5),
            (0.45, 0.5),
            (-0.05, -0.1),
            (0.05, 0.1),
            (0.15, 0.2),
            (0.25, 0.3),
            (0.35, 0.4),
            (-0.35, -0.4),
            (1.05, 1.1),
            (1.45, 1.5),
            (-1.45, -1.5),
            (2.25, 2.3),
            (-2.25, -2.3),
            (2.55, 2.6),
            (-2.55, -2.6),
            (2.5500000000000003, 2.6),
            (4.95, 5.0),
            (-4.95, -5.0),
            (4.9999999, 5.0),
            (-9.999999, -10.0),
            (5.0, 5.0),
            (-10.0, -10.0),
            (0.0, 0.0),
            (-0.0, -0.0),
            (0.04, 0.0),
            (-0.04, -0.0),
            (0.04999999, 0.0),
            (-0.04999999, -0.0),
            (3.1234567, 3.1),
            (-1.2345678901234, -1.2),
            (0.30000000000000004, 0.3),
            (1e-06, 0.0),
            (-7.123456789, -7.1),
        ];
        for (input, expected) in cases {
            let actual = round_one_decimal(input);
            assert_eq!(actual, expected, "round {input}");
            assert_eq!(
                actual.is_sign_negative(),
                expected.is_sign_negative(),
                "sign of round {input}"
            );
        }
    }

    #[test]
    fn the_character_prop_is_built_from_the_three_answers() {
        let character = EveCharacter::from_data(95569887, &data(Some(-0.45), true)).expect("built");
        assert_eq!(
            serde_json::to_value(&character).expect("json"),
            serde_json::json!({
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
            })
        );
    }

    #[test]
    fn whole_security_statuses_are_written_as_integers() {
        let json = |status| {
            let character = EveCharacter::from_data(1, &data(status, false)).expect("built");
            serde_json::to_value(&character).expect("json")["securityStatus"].to_string()
        };
        assert_eq!(json(None), "0");
        assert_eq!(json(Some(4.99)), "5");
        assert_eq!(json(Some(-10.0)), "-10");
        assert_eq!(json(Some(-0.01)), "0");
        assert_eq!(json(Some(1.25)), "1.3");
    }

    #[test]
    fn a_birthday_that_does_not_parse_leaves_the_character_out() {
        let mut data = data(None, false);
        data.character.birthday = "yesterday".to_owned();
        assert_eq!(EveCharacter::from_data(1, &data), None);
    }
}
