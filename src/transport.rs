//! Request construction and the [`Fetcher`] seam this plugin routes HTTP calls through.
//!
//! `openligadb`'s public API (`League::list`, `Match::by_league`, `TableTeam::get_bl_table`,
//! ...) calls `reqwest::get` directly through a private `util` module with no way to
//! substitute the transport, so those methods can't be called as-is inside a WASM component
//! (no direct network access). This module rebuilds the same request URLs `openligadb`
//! builds internally and reuses its public model types (`openligadb::models::*`) for
//! deserialization via `serde_json`, replacing only the fetch step.
//!
//! [`Fetcher`] is the seam the real host `fetch` capability plugs into: `HostFetcher`
//! (compiled only for `wasm32`, so it does not appear in non-`wasm32` documentation
//! builds) delegates to `fulltime_plugin_api::host_fetch`, while native tests substitute a
//! fixture-backed implementation — see `AGENTS.md` ("Implementing the plugin").

use serde::de::DeserializeOwned;
use thiserror::Error;

/// Base URL for the `OpenLigaDB` API. Duplicated from `openligadb::constants::API_BASE_URL`,
/// which is a private module and not reusable from outside that crate.
pub const API_BASE_URL: &str = "https://api.openligadb.de";

/// A source of raw HTTP responses. `HostFetcher` is the production implementation
/// (`wasm32` only, routed through the host); tests substitute a fixture-backed
/// implementation instead.
pub trait Fetcher {
    /// Fetches the response body for a GET request to `url`.
    ///
    /// # Errors
    /// Returns [`FetchError::Request`] on a network failure or non-success HTTP status.
    fn fetch(&self, url: &str) -> Result<Vec<u8>, FetchError>;
}

/// Routes fetches through the host's `fetch` capability via
/// `fulltime_plugin_api::host_fetch`.
///
/// Only compiled for `wasm32`: `host_fetch` only links inside a real component
/// instantiated by a host implementing `fulltime-plugin-api`'s `host` WIT interface, so
/// this type would fail to link on a native target. Native builds (this crate's own
/// tests) use a fixture-backed `Fetcher` instead — see `tests/provider.rs`.
#[cfg(target_arch = "wasm32")]
#[derive(Debug, Default, Clone, Copy)]
pub struct HostFetcher;

#[cfg(target_arch = "wasm32")]
impl Fetcher for HostFetcher {
    fn fetch(&self, url: &str) -> Result<Vec<u8>, FetchError> {
        fulltime_plugin_api::host_fetch(url).map_err(|err| FetchError::Request {
            url: url.to_owned(),
            message: err.message,
        })
    }
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
