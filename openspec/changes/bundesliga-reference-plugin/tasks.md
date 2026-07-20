## 1. Crate Setup

- [x] 1.1 Scaffold the WASM component crate (Cargo.toml targeting `wasm32-wasip2` or the
  Component Model target used by `fulltime-plugin-api`, license, CI). `crate-type =
  ["cdylib", "rlib"]` set; actual `wasm32-wasip2` cross-compilation not yet attempted in
  this environment — only native `cargo build`/`test`/`clippy`/`doc` verified so far.
- [x] 1.2 Add dependencies on `openligadb` (pinned version) and `fulltime-plugin-api`

## 2. Transport Shim

- [x] 2.1 Implement request construction matching `openligadb`'s existing HTTP calls
  (`src/transport.rs`: `leagues_url`/`matches_url`/`table_url`, matching the endpoint
  patterns each `openligadb` model's own `impl` builds internally)
- [ ] 2.2 Route requests through the host `fetch` capability instead of a direct HTTP
  client — **blocked**: `fulltime-plugin-api`'s WIT world (`wit/data-provider.wit`) only
  `export`s `data-provider`; it defines no host `fetch` import yet (that's pending
  `Apps/rust`'s `plugin-host-runtime` change). Implemented the seam this will plug into
  instead: `transport::Fetcher`, a trait any transport (host import, or a fixture/HTTP
  client for testing) implements. See `src/transport.rs` and `AGENTS.md`.
- [x] 2.3 Feed responses back into `openligadb`'s existing deserialization/model types
  (`transport::get_list` deserializes via `serde_json` directly into
  `openligadb::models::*`, since `openligadb::util` is a private module and its own
  `reqwest`-based methods can't be called from a WASM component)
- [x] 2.4 Add integration tests for the shim using recorded/fixture HTTP responses (no live
  calls in tests) — `tests/provider.rs`'s `FixtureFetcher`, backed by
  `tests/fixtures/*.json` (the match fixture is `openligadb`'s own
  `data/match-72395.json`)

## 3. Schema Mapping

- [x] 3.1 Implement mapping from `openligadb` league/team types to canonical
  competition/team types (`src/mapping.rs`: `map_competition`, `map_team`)
- [x] 3.2 Implement mapping from `openligadb` match types to canonical fixture/result types
  (`src/mapping.rs`: `map_fixture`)
- [x] 3.3 Implement mapping from `openligadb` table types to canonical standings
  (`src/mapping.rs`: `map_standings`)
- [x] 3.4 Return `schema-mapping-failure` for any upstream shape that can't be represented
  in the canonical schema, rather than partial data (every mapping function returns
  `Result<_, ProviderError>`, erroring instead of defaulting on a missing/invalid field)

## 4. Plugin Interface Implementation

- [ ] 4.1 Implement the data-provider WIT interface (list-competitions, fetch-fixtures,
  fetch-results, fetch-standings, fetch-metadata) against the shim and mapping layers —
  **partially done, not component-wired**: `src/provider.rs` implements all five
  operations as plain Rust functions with matching signatures/semantics, and
  `tests/provider.rs` exercises them. Not yet wired to an actual WASM component export:
  `fulltime-plugin-api`'s `mod bindings` is private, so it re-exports canonical types but
  no `Guest` trait/`export!` macro a downstream plugin could hook into, and there's no
  host runtime yet to build or test a real export against. See `src/provider.rs`'s module
  doc for the follow-up needed once that seam exists.
- [x] 4.2 Write the plugin manifest declaring the OpenLigaDB API host as the sole network
  capability and the targeted schema/interface versions (`manifest.toml`, validated
  against `fulltime_plugin_api::Manifest::parse` in `tests/manifest.rs`)

## 5. Validation

- [ ] 5.1 Run this plugin's path and a direct `openligadb` call side by side for a sample
  of seasons/matchdays, diffing output against the canonical schema — not started; needs
  live network access and is more meaningful once 4.1's component wiring exists
- [ ] 5.2 Document any fields the canonical schema doesn't carry that `openligadb`
  provides, and confirm with the `fulltime-plugin-api` change owner whether the schema
  needs to account for them — not started. One gap already visible from mapping work:
  `openligadb::models::team::Team`/`TableTeam` carry `icon_url` (team logo), which the
  canonical `Team` record has no field for.
