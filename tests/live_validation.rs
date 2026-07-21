//! Side-by-side validation of this plugin's output against `openligadb`'s own
//! direct-integration client, per
//! `openspec/changes/bundesliga-reference-plugin/tasks.md` (5.1).
//!
//! Ignored by default: it makes live requests to `api.openligadb.de`. Run explicitly with
//! `cargo test --test live_validation -- --ignored`.
//!
//! For each endpoint, the plugin's `provider` path (transport shim + host-fetch-shaped
//! `Fetcher`) and `openligadb`'s own `reqwest`-based client methods are queried for the
//! same season, then both are mapped through the same `mapping` functions and compared for
//! equality. Since both sides share the mapping code, this isolates whether the transport
//! shim (URL construction + raw deserialization) reproduces `openligadb`'s own request
//! shape and response handling faithfully.

use fulltime_plugin_bundesliga::mapping::{map_competition, map_standings};
use fulltime_plugin_bundesliga::provider;
use fulltime_plugin_bundesliga::transport::{FetchError, Fetcher};
use openligadb::models::league::League;
use openligadb::models::table::TableTeam;

/// A season/matchday known (as of writing) to have completed fixtures and a settled table.
const LEAGUE: &str = "bl1";
const SEASON: i32 = 2023;

/// Drives `openligadb`'s async client methods to completion on a fresh Tokio runtime.
///
/// `openligadb`'s `reqwest`-based client needs a running Tokio reactor (not just a bare
/// executor), so a minimal future-poller like `pollster` isn't sufficient here.
fn block_on<F: std::future::Future>(future: F) -> F::Output {
    tokio::runtime::Runtime::new()
        .expect("failed to start a Tokio runtime for the direct openligadb call")
        .block_on(future)
}

struct LiveFetcher(reqwest::blocking::Client);

impl LiveFetcher {
    fn new() -> Self {
        Self(reqwest::blocking::Client::new())
    }
}

impl Fetcher for LiveFetcher {
    fn fetch(&self, url: &str) -> Result<Vec<u8>, FetchError> {
        let response = self.0.get(url).send().map_err(|err| FetchError::Request {
            url: url.to_owned(),
            message: err.to_string(),
        })?;
        response
            .bytes()
            .map(|bytes| bytes.to_vec())
            .map_err(|err| FetchError::Request {
                url: url.to_owned(),
                message: err.to_string(),
            })
    }
}

#[test]
#[ignore = "requires live network access to api.openligadb.de"]
fn plugin_competitions_match_direct_openligadb_league_list() {
    let fetcher = LiveFetcher::new();

    let plugin_competitions = provider::list_competitions(&fetcher)
        .expect("plugin list_competitions should succeed against the live API");

    let direct_leagues =
        block_on(League::list()).expect("direct openligadb League::list should succeed");
    let direct_competitions = direct_leagues
        .iter()
        .filter(|league| league.shortcut.as_deref() == Some(LEAGUE))
        .map(map_competition)
        .collect::<Result<Vec<_>, _>>()
        .expect("direct leagues should map to the canonical schema");

    assert_eq!(plugin_competitions, direct_competitions);
}

#[test]
#[ignore = "requires live network access to api.openligadb.de"]
fn plugin_fixtures_match_direct_openligadb_match_list() {
    let fetcher = LiveFetcher::new();
    let competition_id = format!("{LEAGUE}-{SEASON}");

    let plugin_fixtures = provider::fetch_fixtures(&fetcher, &competition_id)
        .expect("plugin fetch_fixtures should succeed against the live API");

    let direct_matches = block_on(openligadb::models::r#match::Match::by_league(
        LEAGUE, SEASON,
    ))
    .expect("direct openligadb Match::by_league should succeed");
    let direct_fixtures = direct_matches
        .iter()
        .map(|m| fulltime_plugin_bundesliga::mapping::map_fixture(m, &competition_id))
        .collect::<Result<Vec<_>, _>>()
        .expect("direct matches should map to the canonical schema");

    assert_eq!(plugin_fixtures, direct_fixtures);
}

#[test]
#[ignore = "requires live network access to api.openligadb.de"]
fn plugin_standings_match_direct_openligadb_table() {
    let fetcher = LiveFetcher::new();
    let competition_id = format!("{LEAGUE}-{SEASON}");

    let plugin_standings = provider::fetch_standings(&fetcher, &competition_id)
        .expect("plugin fetch_standings should succeed against the live API");

    let direct_rows: Vec<TableTeam> = block_on(TableTeam::get_bl_table(LEAGUE, SEASON))
        .expect("direct openligadb TableTeam::get_bl_table should succeed");
    let direct_standings = map_standings(&direct_rows, &competition_id)
        .expect("direct table rows should map to the canonical schema");

    assert_eq!(plugin_standings, direct_standings);
}
