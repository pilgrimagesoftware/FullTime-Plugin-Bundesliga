//! Maps `openligadb`'s response types to `fulltime-plugin-api`'s canonical schema.
//!
//! Every function here returns [`ProviderError::SchemaMappingFailure`] rather than
//! panicking or silently dropping data when an upstream field required by the canonical
//! schema is missing or out of range, per
//! `openspec/changes/bundesliga-reference-plugin/specs/bundesliga-plugin/spec.md`
//! ("Canonical Schema Mapping").

use fulltime_plugin_api::{
    Competition, Fixture, FixtureStatus, ProviderError, SchemaMappingFailure, Score, Standings,
    StandingsGroup, StandingsRow, Team as CanonicalTeam,
};
use openligadb::models::league::League;
use openligadb::models::r#match::Match;
use openligadb::models::table::TableTeam;
use openligadb::models::team::Team;

fn failure(message: impl Into<String>) -> ProviderError {
    ProviderError::SchemaMappingFailure(SchemaMappingFailure {
        message: message.into(),
    })
}

fn require_u16(value: i32, context: impl Fn() -> String) -> Result<u16, ProviderError> {
    u16::try_from(value).map_err(|_| failure(format!("{}: {value} does not fit in u16", context())))
}

/// Maps an `openligadb` team to the canonical schema, requiring `name` and `short_name`
/// (both `Option` on the upstream type, since not every `OpenLigaDB` endpoint always
/// populates them).
///
/// # Errors
/// Returns [`ProviderError::SchemaMappingFailure`] if `name` or `short_name` is absent.
pub fn map_team(team: &Team) -> Result<CanonicalTeam, ProviderError> {
    Ok(CanonicalTeam {
        id: team.id.to_string(),
        name: team
            .name
            .clone()
            .ok_or_else(|| failure(format!("team {} has no teamName", team.id)))?,
        short_name: team
            .short_name
            .clone()
            .ok_or_else(|| failure(format!("team {} has no shortName", team.id)))?,
    })
}

/// Maps an `openligadb` match to a canonical fixture.
///
/// `openligadb`'s "final result" is the `MatchResult` with `type_id == 2`
/// ("Endergebnis"/final score), per the shape observed in
/// `Libs/openligadb/rust/data/match-72395.json` — halftime and other intermediate results
/// (`type_id == 1`, etc.) are not carried into the canonical `score`.
///
/// `openligadb::models::r#match::Match` exposes only `is_finished: bool`, with no
/// live/postponed/cancelled signal, so [`FixtureStatus`] here is only ever `Finished` or
/// `Scheduled` — the other variants are unreachable from this mapping today.
///
/// # Errors
/// Returns [`ProviderError::SchemaMappingFailure`] if kickoff time, either team's name, or
/// a final score value can't be represented in the canonical schema.
pub fn map_fixture(m: &Match, competition_id: &str) -> Result<Fixture, ProviderError> {
    let kickoff = m
        .when_utc
        .clone()
        .ok_or_else(|| failure(format!("match {} has no matchDateTimeUTC", m.id)))?;

    let status = if m.is_finished {
        FixtureStatus::Finished
    } else {
        FixtureStatus::Scheduled
    };

    let score = m
        .results
        .as_ref()
        .and_then(|results| results.iter().find(|r| r.type_id == 2))
        .map(|final_result| {
            let match_id = m.id;
            let home = final_result.points_team1.ok_or_else(|| {
                failure(format!("match {match_id} final result missing pointsTeam1"))
            })?;
            let away = final_result.points_team2.ok_or_else(|| {
                failure(format!("match {match_id} final result missing pointsTeam2"))
            })?;
            Ok::<_, ProviderError>(Score {
                home: require_u16(home, || format!("match {match_id} pointsTeam1"))?,
                away: require_u16(away, || format!("match {match_id} pointsTeam2"))?,
            })
        })
        .transpose()?;

    Ok(Fixture {
        id: m.id.to_string(),
        competition_id: competition_id.to_owned(),
        group: m.group.name.clone(),
        kickoff,
        home_team: map_team(&m.team1)?,
        away_team: map_team(&m.team2)?,
        venue: m.location.as_ref().and_then(|loc| loc.stadium.clone()),
        status,
        score,
    })
}

/// Maps an `openligadb` league table into canonical standings, as a single unnamed group.
///
/// The Bundesliga table is a single-table format, not group-based. Rank is derived from
/// the response's row order (`getbltable` returns rows pre-sorted by position), since
/// `TableTeam` carries no explicit rank field.
///
/// # Errors
/// Returns [`ProviderError::SchemaMappingFailure`] if any row can't be mapped to the
/// canonical schema.
pub fn map_standings(rows: &[TableTeam], competition_id: &str) -> Result<Standings, ProviderError> {
    let rows = rows
        .iter()
        .enumerate()
        .map(|(index, row)| map_standings_row(row, index))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(Standings {
        competition_id: competition_id.to_owned(),
        groups: vec![StandingsGroup { name: None, rows }],
    })
}

fn map_standings_row(row: &TableTeam, index: usize) -> Result<StandingsRow, ProviderError> {
    let context = |field: &str| format!("table row {}: {field}", row.id);
    Ok(StandingsRow {
        team: CanonicalTeam {
            id: row.id.to_string(),
            name: row
                .name
                .clone()
                .ok_or_else(|| failure(format!("table row {} has no teamName", row.id)))?,
            short_name: row
                .short_name
                .clone()
                .ok_or_else(|| failure(format!("table row {} has no shortName", row.id)))?,
        },
        #[allow(clippy::cast_possible_truncation)]
        rank: index as u16 + 1,
        played: require_u16(row.matches, || context("matches"))?,
        won: require_u16(row.wins, || context("won"))?,
        drawn: require_u16(row.draws, || context("draw"))?,
        lost: require_u16(row.losses, || context("lost"))?,
        goals_for: require_u16(row.goals, || context("goals"))?,
        goals_against: require_u16(row.opponent_goals, || context("opponentGoals"))?,
        points: require_u16(row.points, || context("points"))?,
    })
}

/// Maps an `openligadb` league (filtered by shortcut) to a canonical competition.
///
/// Uses the `{shortcut}-{season}` identifier convention this plugin uses throughout
/// (matching `fulltime-plugin-api`'s `tests/bundesliga_shape.rs` fixture, e.g.
/// `"bl1-2024"`).
///
/// # Errors
/// Returns [`ProviderError::SchemaMappingFailure`] if the league has no name or its season
/// isn't a valid integer.
pub fn map_competition(league: &League) -> Result<Competition, ProviderError> {
    let season = league
        .season
        .as_deref()
        .ok_or_else(|| failure(format!("league {} has no leagueSeason", league.id)))?;
    let name = league
        .name
        .clone()
        .ok_or_else(|| failure(format!("league {} has no leagueName", league.id)))?;
    let shortcut = league
        .shortcut
        .as_deref()
        .ok_or_else(|| failure(format!("league {} has no leagueShortcut", league.id)))?;

    Ok(Competition {
        id: format!("{shortcut}-{season}"),
        name,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_a_finished_match_fixture_to_a_canonical_fixture_with_final_score() {
        let raw = include_str!("../tests/fixtures/match-72395.json");
        let m: Match = serde_json::from_str(raw).unwrap();

        let fixture = map_fixture(&m, "bl1-2024").unwrap();

        assert_eq!(fixture.id, "72395");
        assert_eq!(fixture.status, FixtureStatus::Finished);
        assert_eq!(fixture.group.as_deref(), Some("21. Spieltag"));
        assert_eq!(fixture.home_team.short_name, "Leipzig");
        assert_eq!(fixture.away_team.short_name, "St. Pauli");
        // The fixture carries a halftime (type_id 1) and a final (type_id 2) result; only
        // the final score maps into the canonical `score`.
        let score = fixture.score.unwrap();
        assert_eq!((score.home, score.away), (2, 0));
    }
}
