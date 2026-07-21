## Context

`openligadb` (`Libs/openligadb/rust`) already implements the HTTP calls and response types
for the OpenLigaDB API (leagues, seasons, matches, tables — see `src/models/`). This plugin
is a thin wrapper: it compiles to a WASM component, calls `openligadb`'s existing client
logic through the host-provided `fetch` capability (plugins have no direct network access,
per the umbrella design), and maps `openligadb`'s response types to
`fulltime-plugin-api`'s canonical schema.

## Goals / Non-Goals

**Goals:**
- Prove `fulltime-plugin-api`'s WIT interface and canonical schema are sufficient for a
  real, currently-shipping data source.
- Reuse `openligadb`'s existing parsing/model logic rather than rewriting it against the
  host's `fetch` capability from scratch.
- Produce output that is verifiably equivalent to `openligadb`'s current direct-integration
  output, so the eventual app cutover is a swap, not a behavior change.

**Non-Goals:**
- Changing `openligadb`'s public API or response types — this plugin adapts to them, it
  doesn't ask them to change.
- Implementing the plugin host runtime or the app cutover — those live in `Apps/rust`'s
  `plugin-host-runtime` change.
- Supporting any competition other than Bundesliga — EPL and national-team plugins are
  separate future work once this reference plugin validates the API.

## Decisions

**`openligadb`'s HTTP client calls are replaced with calls through the host's `fetch`
capability, but its response parsing/model types are reused unchanged.**
Rationale: `openligadb` was written assuming direct network access; inside a WASM
component it has none. Rather than forking `openligadb` or making it capability-aware,
this plugin implements a thin transport shim: it builds the same requests `openligadb`
would, sends them through the host `fetch` import, and feeds the raw response bytes back
into `openligadb`'s existing deserialization. This isolates the change to a request/response
plumbing layer instead of touching `openligadb`'s tested parsing logic.

**Schema mapping (`openligadb` types → canonical schema) lives entirely in this plugin, not
in `openligadb` itself.**
Rationale: `openligadb` is a general-purpose OpenLigaDB API client with its own consumers
outside this plugin system; it should not carry a dependency on `fulltime-plugin-api`'s
schema. The mapping is this plugin's job, per the umbrella design's decision that the
canonical schema is owned by the host, not by any single source.

**Manifest declares exactly one network host: `openligadb`'s API host.**
Rationale: matches the principle of least privilege from the umbrella design's capability
model — this plugin has no legitimate reason to reach any other host.

## Risks / Trade-offs

- [Behavior drift between this plugin's output and `openligadb`'s current direct-integration
  output] → Run both paths side by side and diff output against the canonical schema before
  either the umbrella app-cutover work or this plugin is considered validated.
- [WASM component tooling for Rust is still maturing] → Follow the pinned `wit-bindgen`/
  `wasmtime`-toolchain versions documented in `fulltime-plugin-api`, don't pin independently
  here.
- [Transport shim (host `fetch` → `openligadb` parsing) is untested code not exercised by
  `openligadb`'s own test suite] → Add integration tests in this repo specifically for the
  shim, using recorded/fixture HTTP responses rather than live calls.

## Migration Plan

Not applicable — this is a new plugin with no prior version. It becomes load-bearing only
once `Apps/rust`'s `plugin-host-runtime` change cuts the app over to it, which is
coordinated separately.

## Open Questions

- Should this plugin vendor a pinned `openligadb` version, or track its latest release?
  Leaning toward a pinned version bumped deliberately, consistent with keeping the
  diffing-against-direct-integration validation meaningful.

## Fields `openligadb` Provides That the Canonical Schema Doesn't Carry

Task 5.2's side-by-side validation (`tests/live_validation.rs`, run against the live API for
`bl1`/2023) confirmed output equivalence for every field the canonical schema does carry. The
following upstream fields have no canonical-schema home today, so they're dropped by
`src/mapping.rs` rather than mismapped:

- **`Team`/`TableTeam::icon_url`** (`teamIconUrl`) — a team logo URL. Present on both match
  team payloads and table rows.
- **`Team::group`** (`teamGroupName`) — an upstream grouping label for the team, distinct
  from `Match::group` (matchday/round grouping, which the canonical `Fixture::group` does
  carry).
- **`League::sport`** — the sport this league belongs to (always football for this plugin,
  but not implied by the canonical `Competition` type itself).
- **`Match`'s league-identifying fields** (`league_id`, `league_name`, `league_season`,
  `league_shortcut`, `time_zone`) — redundant with `Fixture::competition_id` for this
  plugin's single-league scope, but would matter for a multi-league canonical consumer.
- **`Match::last_update`** — no canonical "last modified" timestamp exists on `Fixture`.
- **`Match::results`'s non-final entries** (halftime, etc., `type_id != 2`) — only the final
  score maps to `Fixture::score`; intermediate results are dropped entirely.
- **`Match::goals`** (individual goal events: scorer, minute, penalty/own-goal flags) — no
  equivalent field exists on the canonical `Fixture`.
- **`Location::city`** — only `Location::stadium` maps to `Fixture::venue`; the city isn't
  carried.

None of these gaps block this change: every canonical-schema field is fully populated and
verified equivalent to `openligadb`'s direct output. Whether `fulltime-plugin-api`'s schema
should grow fields for any of these (team logos and goal-scorer events are the two most
likely candidates for a future UI need) is a decision for that repo's change owner, not this
plugin — tracked in
[pilgrimagesoftware/fulltime-plugin-api#10](https://github.com/pilgrimagesoftware/fulltime-plugin-api/issues/10).
