//! Implements `fulltime-plugin-api`'s five `data-provider` operations against the
//! transport shim and mapping layer.
//!
//! Takes a `&dyn Fetcher` parameter rather than reading a host capability directly, so
//! these functions are exercised in `tests/provider.rs` against fixture data without a
//! real component. `crate::component` wires them into the actual WASM component export
//! (`fulltime_plugin_api::Guest`/`export!`), passing `transport::HostFetcher` in place of
//! a fixture — see that module and `AGENTS.md` ("Implementing the plugin").

use fulltime_plugin_api::{
    Competition, Fixture, FixtureStatus, NetworkFailure, ProviderError, SchemaMappingFailure,
    Standings,
};
use openligadb::models::league::League;
use openligadb::models::r#match::Match;
use openligadb::models::table::TableTeam;

use crate::mapping::{map_competition, map_fixture, map_standings};
use crate::transport::{self, FetchError, Fetcher};

/// This plugin serves exactly one `OpenLigaDB` league shortcut: the top-tier Bundesliga.
const LEAGUE_SHORTCUT: &str = "bl1";

/// Lists the Bundesliga competitions (one per season) this plugin can currently supply
/// data for, by fetching the full `OpenLigaDB` league catalog and filtering to this
/// plugin's league shortcut (`bl1`).
///
/// # Errors
/// Returns [`ProviderError::NetworkFailure`] on a transport failure, or
/// [`ProviderError::SchemaMappingFailure`] if a matching league can't be mapped to the
/// canonical schema.
pub fn list_competitions(fetcher: &dyn Fetcher) -> Result<Vec<Competition>, ProviderError> {
    let leagues: Vec<League> =
        transport::get_list(fetcher, &transport::leagues_url()).map_err(|e| network_failure(&e))?;

    leagues
        .iter()
        .filter(|league| league.shortcut.as_deref() == Some(LEAGUE_SHORTCUT))
        .map(map_competition)
        .collect()
}

/// Fetches all fixtures (scheduled and finished) for `competition_id`.
///
/// # Errors
/// Returns [`ProviderError::SchemaMappingFailure`] if `competition_id` isn't a
/// `{league}-{season}` id this plugin issued, [`ProviderError::NetworkFailure`] on a
/// transport failure, or [`ProviderError::SchemaMappingFailure`] if a match can't be
/// mapped to the canonical schema.
pub fn fetch_fixtures(
    fetcher: &dyn Fetcher,
    competition_id: &str,
) -> Result<Vec<Fixture>, ProviderError> {
    let season = parse_season(competition_id)?;
    let url = transport::matches_url(LEAGUE_SHORTCUT, season);
    let matches: Vec<Match> =
        transport::get_list(fetcher, &url).map_err(|e| network_failure(&e))?;

    matches
        .iter()
        .map(|m| map_fixture(m, competition_id))
        .collect()
}

/// Fetches results (finished matches only) for `competition_id`.
///
/// # Errors
/// See [`fetch_fixtures`].
pub fn fetch_results(
    fetcher: &dyn Fetcher,
    competition_id: &str,
) -> Result<Vec<Fixture>, ProviderError> {
    Ok(fetch_fixtures(fetcher, competition_id)?
        .into_iter()
        .filter(|fixture| fixture.status == FixtureStatus::Finished)
        .collect())
}

/// Fetches the Bundesliga table for `competition_id`.
///
/// # Errors
/// Returns [`ProviderError::SchemaMappingFailure`] if `competition_id` isn't a
/// `{league}-{season}` id this plugin issued, [`ProviderError::NetworkFailure`] on a
/// transport failure, or [`ProviderError::SchemaMappingFailure`] if a row can't be mapped
/// to the canonical schema.
pub fn fetch_standings(
    fetcher: &dyn Fetcher,
    competition_id: &str,
) -> Result<Standings, ProviderError> {
    let season = parse_season(competition_id)?;
    let url = transport::table_url(LEAGUE_SHORTCUT, season);
    let rows: Vec<TableTeam> =
        transport::get_list(fetcher, &url).map_err(|e| network_failure(&e))?;

    map_standings(&rows, competition_id)
}

/// Fetches metadata for `competition_id` by filtering [`list_competitions`]'s output down
/// to the matching id.
///
/// # Errors
/// Returns [`ProviderError::SchemaMappingFailure`] if `competition_id` doesn't match any
/// competition this plugin currently lists, or the errors documented on
/// [`list_competitions`].
pub fn fetch_metadata(
    fetcher: &dyn Fetcher,
    competition_id: &str,
) -> Result<Competition, ProviderError> {
    list_competitions(fetcher)?
        .into_iter()
        .find(|competition| competition.id == competition_id)
        .ok_or_else(|| {
            ProviderError::SchemaMappingFailure(SchemaMappingFailure {
                message: format!("no competition metadata for {competition_id}"),
            })
        })
}

/// Parses the season out of this plugin's `{league}-{season}` competition id convention
/// (e.g. `"bl1-2024"`), rejecting any id not issued for [`LEAGUE_SHORTCUT`].
fn parse_season(competition_id: &str) -> Result<i32, ProviderError> {
    let invalid = || {
        ProviderError::SchemaMappingFailure(SchemaMappingFailure {
            message: format!(
                "competition id {competition_id} is not a valid {LEAGUE_SHORTCUT}-<season> id"
            ),
        })
    };

    let season = competition_id
        .strip_prefix(LEAGUE_SHORTCUT)
        .and_then(|rest| rest.strip_prefix('-'))
        .ok_or_else(invalid)?;

    season.parse::<i32>().map_err(|_| invalid())
}

fn network_failure(err: &FetchError) -> ProviderError {
    ProviderError::NetworkFailure(NetworkFailure {
        message: err.to_string(),
    })
}
