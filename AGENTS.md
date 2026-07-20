# AGENTS.md

This file provides guidance to Claude Code (claude.ai/code), Codex (openai.com/codex/), GitHub
Copilot (copilot.github.com), and other models when working with code in this repository.

## About This Project

This is the Bundesliga reference data-provider plugin for the FullTime project. It is a WASM
component implementing [`fulltime-plugin-api`](https://github.com/pilgrimagesoftware/fulltime-plugin-api)'s
`data-provider` interface, wrapping the existing [`openligadb`](https://github.com/pilgrimagesoftware/openligadb.rs)
crate's fetch logic rather than reimplementing HTTP calls against the OpenLigaDB API.

It is the first plugin the FullTime desktop app's (`Apps/rust`) plugin host runtime loads, and
serves as the reference implementation proving the `fulltime-plugin-api` contract is sufficient
before any second plugin (EPL, national teams) is attempted.

## Repository

- **GitHub**: https://github.com/pilgrimagesoftware/FullTime-Plugin-Bundesliga
- **Crate name**: `fulltime-plugin-bundesliga`
- **Depends on**: `fulltime-plugin-api` (WIT interface, canonical schema, manifest format),
  `openligadb` (wrapped fetch logic)

## Branches and Workflow

- `develop` is the source of truth for active development (Git Flow).
- Feature branches are cut from `develop` and merged back via pull request.
- `master` tracks released, published versions.
- See [RELEASING.md](RELEASING.md) for how versions are cut.

## Coding Conventions

- All public items **must** have doc comments (`///`).
- Follow the shared [`docs/rust.md`](../../docs/rust.md) conventions from the umbrella `FullTime`
  repo: `#![deny(unsafe_code)]` unless a documented `# Safety` invariant requires otherwise, no
  `unwrap()`/`expect()` outside tests, structured errors via `thiserror`.
- YAML files use the `.yaml` extension (not `.yml`).
- Commit messages follow [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/)
  (`feat:`, `fix:`, `chore:`, `docs:`, etc.) — `git-cliff` reads these to generate the changelog
  and compute the release version.

## Implementing the plugin

- The `data-provider` interface and canonical schema are defined in `fulltime-plugin-api`, not
  here — see that repo's `docs/plugin-authoring.md` for the contract and versioning policy.
- Map `openligadb`'s response types (`Team`, `Match`, `TableTeam`) to the canonical schema
  (`Team`, `Fixture`, `Standings`) in `src/mapping.rs`; do not reimplement HTTP fetch logic that
  already exists in `openligadb`.
- The plugin manifest declares `api.openligadb.de` as the sole network host.
- See `openspec/changes/bundesliga-reference-plugin/tasks.md` for the current task breakdown.

## Running Checks Locally

```bash
cargo build
cargo test --all-features --workspace
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --check
```
