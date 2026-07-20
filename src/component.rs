//! WASM component export: implements `fulltime_plugin_api::Guest` for the `data-provider`
//! interface, wiring `provider`'s plain-function operations to the real host `fetch`
//! capability via `transport::HostFetcher`.
//!
//! Compiled only for `wasm32` — [`fulltime_plugin_api::export`] and
//! `transport::HostFetcher` only make sense as part of a real component instantiated by a
//! host; native builds (this crate's own tests) exercise `provider`'s functions directly
//! against fixture data instead — see `tests/provider.rs`.

#![cfg(target_arch = "wasm32")]

use fulltime_plugin_api::{Competition, Fixture, ProviderError, Standings};

use crate::provider;
use crate::transport::HostFetcher;

/// Zero-sized type implementing the `data-provider` interface's `Guest` trait.
struct BundesligaPlugin;

impl fulltime_plugin_api::Guest for BundesligaPlugin {
    fn list_competitions() -> Result<Vec<Competition>, ProviderError> {
        provider::list_competitions(&HostFetcher)
    }

    fn fetch_fixtures(competition_id: String) -> Result<Vec<Fixture>, ProviderError> {
        provider::fetch_fixtures(&HostFetcher, &competition_id)
    }

    fn fetch_results(competition_id: String) -> Result<Vec<Fixture>, ProviderError> {
        provider::fetch_results(&HostFetcher, &competition_id)
    }

    fn fetch_standings(competition_id: String) -> Result<Standings, ProviderError> {
        provider::fetch_standings(&HostFetcher, &competition_id)
    }

    fn fetch_metadata(competition_id: String) -> Result<Competition, ProviderError> {
        provider::fetch_metadata(&HostFetcher, &competition_id)
    }
}

fulltime_plugin_api::export!(BundesligaPlugin with_types_in fulltime_plugin_api);
