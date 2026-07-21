# FullTime-Plugin-Bundesliga

[![Crates.io](https://img.shields.io/crates/v/fulltime-plugin-bundesliga.svg)](https://crates.io/crates/fulltime-plugin-bundesliga)
[![docs.rs](https://img.shields.io/docsrs/fulltime-plugin-bundesliga)](https://docs.rs/fulltime-plugin-bundesliga)
[![CI](https://github.com/pilgrimagesoftware/FullTime-Plugin-Bundesliga/actions/workflows/ci.yaml/badge.svg)](https://github.com/pilgrimagesoftware/FullTime-Plugin-Bundesliga/actions/workflows/ci.yaml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

The Bundesliga reference data-provider plugin for [FullTime](https://github.com/pilgrimagesoftware/FullTime). A WASM
component implementing [`fulltime-plugin-api`](https://github.com/pilgrimagesoftware/fulltime-plugin-api)'s
`data-provider` interface, wrapping [`openligadb`](https://github.com/pilgrimagesoftware/openligadb.rs)'s existing
fetch logic rather than reimplementing HTTP calls against the OpenLigaDB API.

This is the first plugin the FullTime desktop app's plugin host runtime (`Apps/rust`) loads, and serves as the
reference implementation proving the `fulltime-plugin-api` contract is sufficient before any second plugin (EPL,
national teams) is attempted.

## What this plugin does

- Wraps `openligadb`'s fetch logic for leagues, seasons, matches, and tables.
- Maps `openligadb`'s response types to `fulltime-plugin-api`'s canonical schema (competitions, teams, fixtures,
  results, standings).
- Ships a manifest declaring `api.openligadb.de` as its sole network capability, and the schema/interface versions
  it targets.

## Status

Scaffolding in progress — see `openspec/changes/bundesliga-reference-plugin/` for the proposal, design, and task
breakdown, and [`FullTime`](https://github.com/pilgrimagesoftware/FullTime)'s
`openspec/changes/league-data-plugin-system` for the umbrella plugin architecture change this plugin is part of.

## Building a plugin against this contract

This repo is the reference implementation, not the contract itself. See
[`fulltime-plugin-api`](https://github.com/pilgrimagesoftware/fulltime-plugin-api)'s
`docs/plugin-authoring.md` for the WIT interface, manifest format, and versioning policy any plugin (including
this one) implements against.

## Change log

[CHANGELOG](CHANGELOG.md)

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for the development workflow and commit conventions, and
[RELEASING.md](RELEASING.md) for how versions get cut.
This project follows the [Code of Conduct](CODE_OF_CONDUCT.md).

## Security

See [SECURITY.md](SECURITY.md) to report a vulnerability.

## License

Licensed under:

* MIT license ([LICENSE](LICENSE) or <https://opensource.org/licenses/MIT>)

## Contribution

Unless explicitly stated otherwise, any contribution intentionally submitted for inclusion in the work by you shall
be licensed as above.
