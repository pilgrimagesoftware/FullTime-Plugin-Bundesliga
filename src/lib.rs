//! Bundesliga reference data-provider plugin for `FullTime`.
//!
//! Wraps [`openligadb`]'s existing `OpenLigaDB` API models against
//! [`fulltime_plugin_api`]'s canonical `league-data-schema`, routing HTTP requests through
//! the [`transport::Fetcher`] seam rather than `openligadb`'s own (WASM-incompatible)
//! direct `reqwest` calls. See `AGENTS.md` for the repo's role in the `FullTime` plugin
//! architecture and what's implemented so far.

#![warn(clippy::pedantic, clippy::nursery, missing_docs, rust_2018_idioms)]
#![deny(unsafe_op_in_unsafe_fn)]
#![forbid(unsafe_code)]

pub mod mapping;
pub mod provider;
pub mod transport;

pub use transport::{FetchError, Fetcher};
