//! Integration tests for the `provider` operations against recorded fixture responses —
//! no live calls, per
//! `openspec/changes/bundesliga-reference-plugin/tasks.md` (2.4).

use std::collections::HashMap;

use fulltime_plugin_api::FixtureStatus;
use fulltime_plugin_bundesliga::provider;
use fulltime_plugin_bundesliga::transport::{self, FetchError, Fetcher};

struct FixtureFetcher {
    responses: HashMap<String, Vec<u8>>,
}

impl FixtureFetcher {
    fn new() -> Self {
        let mut responses = HashMap::new();
        responses.insert(
            transport::leagues_url(),
            include_bytes!("fixtures/leagues.json").to_vec(),
        );
        responses.insert(
            transport::matches_url("bl1", 2024),
            include_bytes!("fixtures/matches-bl1-2024.json").to_vec(),
        );
        responses.insert(
            transport::table_url("bl1", 2024),
            include_bytes!("fixtures/table-bl1-2024.json").to_vec(),
        );
        Self { responses }
    }
}

impl Fetcher for FixtureFetcher {
    fn fetch(&self, url: &str) -> Result<Vec<u8>, FetchError> {
        self.responses
            .get(url)
            .cloned()
            .ok_or_else(|| FetchError::Request {
                url: url.to_owned(),
                message: "no fixture registered for this URL".to_owned(),
            })
    }
}

#[test]
fn lists_bundesliga_competitions_filtered_from_the_full_league_catalog() {
    let fetcher = FixtureFetcher::new();

    let competitions = provider::list_competitions(&fetcher).unwrap();

    assert_eq!(competitions.len(), 1);
    assert_eq!(competitions[0].id, "bl1-2024");
    assert_eq!(competitions[0].name, "1. Fußball-Bundesliga 2024/2025");
}

#[test]
fn fetch_fixtures_maps_openligadb_matches_to_canonical_fixtures() {
    let fetcher = FixtureFetcher::new();

    let fixtures = provider::fetch_fixtures(&fetcher, "bl1-2024").unwrap();

    assert_eq!(fixtures.len(), 1);
    let fixture = &fixtures[0];
    assert_eq!(fixture.id, "72395");
    assert_eq!(fixture.competition_id, "bl1-2024");
    assert_eq!(fixture.status, FixtureStatus::Finished);
    assert_eq!(fixture.home_team.short_name, "Leipzig");
    assert_eq!(fixture.away_team.short_name, "St. Pauli");
    let score = fixture.score.as_ref().unwrap();
    assert_eq!((score.home, score.away), (2, 0));
}

#[test]
fn fetch_results_only_returns_finished_fixtures() {
    let fetcher = FixtureFetcher::new();

    let results = provider::fetch_results(&fetcher, "bl1-2024").unwrap();

    assert_eq!(results.len(), 1);
    assert!(results.iter().all(|f| f.status == FixtureStatus::Finished));
}

#[test]
fn fetch_standings_maps_the_table_into_a_single_ranked_group() {
    let fetcher = FixtureFetcher::new();

    let standings = provider::fetch_standings(&fetcher, "bl1-2024").unwrap();

    assert_eq!(standings.competition_id, "bl1-2024");
    assert_eq!(standings.groups.len(), 1);
    assert!(standings.groups[0].name.is_none());
    let rows = &standings.groups[0].rows;
    assert_eq!(rows[0].rank, 1);
    assert_eq!(rows[0].team.short_name, "FCB");
    assert_eq!(rows[0].points, 9);
    assert_eq!(rows[1].rank, 2);
}

#[test]
fn fetch_metadata_returns_the_matching_competition() {
    let fetcher = FixtureFetcher::new();

    let competition = provider::fetch_metadata(&fetcher, "bl1-2024").unwrap();

    assert_eq!(competition.id, "bl1-2024");
}

#[test]
fn rejects_a_competition_id_this_plugin_did_not_issue() {
    let fetcher = FixtureFetcher::new();

    let err = provider::fetch_fixtures(&fetcher, "epl-2024").unwrap_err();

    assert!(matches!(
        err,
        fulltime_plugin_api::ProviderError::SchemaMappingFailure(_)
    ));
}
