## Why

The plugin architecture (umbrella change `FullTime#1`,
`openspec/changes/league-data-plugin-system`) needs a real, working plugin to prove the
`fulltime-plugin-api` contract is sufficient before any second plugin is attempted. The
Bundesliga plugin is that reference implementation: it wraps the existing `openligadb`
crate (`Libs/openligadb/rust`) rather than rewriting its HTTP logic, and is the first
plugin `fulltime-core`'s host runtime loads.

## What Changes

- Scaffold this repo as a WASM component crate implementing `fulltime-plugin-api`'s
  data-provider WIT interface.
- Wrap `openligadb`'s existing fetch logic (leagues, seasons, matches, tables) rather than
  reimplementing HTTP calls against the OpenLigaDB API.
- Implement mapping from `openligadb`'s response types to `fulltime-plugin-api`'s canonical
  schema (competitions, teams, fixtures, results, standings).
- Write the plugin manifest declaring `openligadb`'s API host as the sole network
  capability, and the schema/interface versions this plugin targets.
- Validate plugin output against the canonical schema by running this plugin's path
  side by side with `openligadb`'s direct output and diffing the two.

## Capabilities

### New Capabilities

- `bundesliga-plugin`: a `fulltime-plugin-api`-conformant WASM component supplying
  Bundesliga fixtures, results, standings, and team/competition metadata via `openligadb`.

### Modified Capabilities

- (none — new repo, no existing specs predate this change)

## Impact

- **This repo (`FullTime-Plugin-Bundesliga`)**: net-new. Builds to a WASM component
  bundled with `FullTime.rs` as a first-party plugin.
- **`Libs/openligadb/rust`**: consumed as a dependency, unchanged — this plugin wraps it
  rather than modifying it.
- Depends on `fulltime-plugin-api`'s `define-league-data-contract` change for the WIT
  interface, canonical schema, and manifest format this plugin implements against.
- Depends on `Apps/rust`'s `plugin-host-runtime` change existing (or at least its manifest
  format/loading contract being stable) for this plugin to actually be loaded and run
  end-to-end; this change can be built and unit-tested independently in the meantime.
- Out of scope: cutting `fulltime-core` over to load this plugin instead of any direct
  `openligadb` dependency — that cutover is tracked in `Apps/rust`'s
  `plugin-host-runtime` change, coordinated once both sides are ready.
