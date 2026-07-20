//! Request construction and the [`Fetcher`] seam this plugin routes HTTP calls through.
//!
//! `openligadb`'s public API (`League::list`, `Match::by_league`, `TableTeam::get_bl_table`,
//! ...) calls `reqwest::get` directly through a private `util` module with no way to
//! substitute the transport, so those methods can't be called as-is inside a WASM component
//! (no direct network access). This module rebuilds the same request URLs `openligadb`
//! builds internally and reuses its public model types (`openligadb::models::*`) for
//! deserialization via `serde_json`, replacing only the fetch step.
//!
//! [`Fetcher`] is the seam a future host `fetch` capability will be wired into once
//! `fulltime-plugin-api`'s WIT contract defines that import — see `AGENTS.md`
//! ("Implementing the plugin").

use serde::de::DeserializeOwned;
use thiserror::Error;

/// Base URL for the `OpenLigaDB` API. Duplicated from `openligadb::constants::API_BASE_URL`,
/// which is a private module and not reusable from outside that crate.
pub const API_BASE_URL: &str = "https://api.openligadb.de";

/// A source of raw HTTP responses. The host runtime will implement this over its `fetch`
/// capability once that WIT import exists; until then, tests substitute a fixture-backed
/// implementation.
pub trait Fetcher {
    /// Fetches the response body for a GET request to `url`.
    ///
    /// # Errors
    /// Returns [`FetchError::Request`] on a network failure or non-success HTTP status.
    fn fetch(&self, url: &str) -> Result<Vec<u8>, FetchError>;
}

/// A transport-level failure fetching or parsing an upstream response.
#[derive(Debug, Error)]
pub enum FetchError {
    /// The request itself failed (network error, non-2xx status, etc).
    #[error("request to {url} failed: {message}")]
    Request {
        /// The URL that was requested.
        url: String,
        /// A human-readable description of the failure.
        message: String,
    },
    /// The response body could not be deserialized into the expected type.
    #[error("failed to parse response from {url}: {source}")]
    Deserialize {
        /// The URL that was requested.
        url: String,
        /// The underlying deserialization error.
        #[source]
        source: serde_json::Error,
    },
}

/// Fetches and deserializes a list of values from `url` using `fetcher`.
pub(crate) fn get_list<M: DeserializeOwned>(
    fetcher: &dyn Fetcher,
    url: &str,
) -> Result<Vec<M>, FetchError> {
    let bytes = fetcher.fetch(url)?;
    serde_json::from_slice(&bytes).map_err(|source| FetchError::Deserialize {
        url: url.to_owned(),
        source,
    })
}

/// URL for the available-leagues endpoint (`getavailableleagues`), matching
/// `openligadb::models::league::League::list`'s request shape.
#[must_use]
pub fn leagues_url() -> String {
    format!("{API_BASE_URL}/getavailableleagues")
}

/// URL for the Bundesliga table endpoint (`getbltable/{league}/{season}`), matching
/// `openligadb::models::table::TableTeam::get_bl_table`'s request shape.
#[must_use]
pub fn table_url(league: &str, season: i32) -> String {
    format!("{API_BASE_URL}/getbltable/{league}/{season}")
}

/// URL for a league's season matches (`getmatchdata/{league}/{season}`), matching
/// `openligadb::models::match::Match::by_league`'s request shape.
#[must_use]
pub fn matches_url(league: &str, season: i32) -> String {
    format!("{API_BASE_URL}/getmatchdata/{league}/{season}")
}
