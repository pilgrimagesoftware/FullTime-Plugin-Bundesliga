## 1. Crate Setup

- [x] 1.1 Scaffold the WASM component crate (Cargo.toml targeting `wasm32-wasip2` or the
  Component Model target used by `fulltime-plugin-api`, license, CI). `crate-type =
  ["cdylib", "rlib"]` set. `cargo check --target wasm32-wasip2` on this crate directly
  fails — not from anything in this crate's own code, but because `openligadb`'s hard
  `reqwest`/`tokio` dependency (its TLS stack, `aws-lc-sys`) can't cross-compile to
  `wasm32-wasip2` in this environment (`clang` can't find `wasm32-wasi` libc headers for
  `aws-lc-sys`'s C sources). Verified this crate's own WASM component code (the `Guest`
  impl + `export!` call in `src/component.rs`) is correct by cross-compiling an isolated
  scratch crate against `fulltime-plugin-api` alone (no `openligadb`) for
  `wasm32-wasip2` — it built clean. See task 2.2's note; fixing `openligadb` to gate
  `reqwest` behind an optional feature is out of scope for this repo.
- [x] 1.2 Add dependencies on `openligadb` (pinned version) and `fulltime-plugin-api`

## 2. Transport Shim

- [x] 2.1 Implement request construction matching `openligadb`'s existing HTTP calls
  (`src/transport.rs`: `leagues_url`/`matches_url`/`table_url`, matching the endpoint
  patterns each `openligadb` model's own `impl` builds internally)
- [x] 2.2 Route requests through the host `fetch` capability instead of a direct HTTP
  client. Was blocked on `fulltime-plugin-api`'s WIT world defining no host `fetch`
  import — unblocked by
  [pilgrimagesoftware/fulltime-plugin-api#7](https://github.com/pilgrimagesoftware/fulltime-plugin-api/pull/7)
  (`add-host-fetch-capability`), which adds `interface host { fetch: ... }` and
  `import host;` to `world plugin`. `transport::HostFetcher` (`wasm32`-only) now
  implements `Fetcher` by delegating to `fulltime_plugin_api::host_fetch`; native tests
  keep using a fixture-backed `Fetcher`. PR #7 merged and released as `fulltime-plugin-api`
  `0.1.1` (not `0.2.0` — a squash-merge dropped the `feat!:` prefix, under-bumping the
  version; see that crate's `RELEASING.md`). `Cargo.toml` now depends on
  `fulltime-plugin-api = "0.1.1"` from crates.io, no longer the `git`/`TEMPORARY` pin.
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

- [x] 4.1 Implement the data-provider WIT interface (list-competitions, fetch-fixtures,
  fetch-results, fetch-standings, fetch-metadata) against the shim and mapping layers.
  Was blocked on `fulltime-plugin-api` not re-exporting a `Guest` trait/`export!` macro —
  unblocked by the same PR as 2.2. `src/component.rs` implements
  `fulltime_plugin_api::Guest` (`wasm32`-only), delegating to `provider`'s functions with
  `transport::HostFetcher` as the fetcher, and calls
  `fulltime_plugin_api::export!(BundesligaPlugin with_types_in fulltime_plugin_api)` —
  note the `with_types_in` form is required from a downstream crate (the single-arg form
  only resolves inside `fulltime-plugin-api` itself), discovered and documented in that
  PR's design.md. Verified via the same isolated scratch-crate cross-compile as task 1.1;
  not yet verified building *this* crate to `wasm32-wasip2` end-to-end, blocked on
  `openligadb`'s `reqwest`/`tokio` dependency (see task 1.1's note) — that's a separate,
  unaddressed blocker in `Libs/openligadb/rust`, not in this repo or
  `fulltime-plugin-api`.
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
