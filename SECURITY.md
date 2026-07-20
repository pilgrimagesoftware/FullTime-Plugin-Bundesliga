# Security Policy

## Supported Versions

This crate is pre-1.0. Only the latest version published on
[crates.io](https://crates.io/crates/fulltime-plugin-bundesliga) receives security fixes. Please upgrade before
reporting an issue to confirm it still reproduces.

## Reporting a Vulnerability

Do not open a public issue for security vulnerabilities.

Report privately via GitHub's
[Security Advisories](https://github.com/pilgrimagesoftware/FullTime-Plugin-Bundesliga/security/advisories/new).
This keeps the report confidential until a fix is released.

Include, where possible:

- Affected version(s)
- A minimal reproduction or proof of concept
- Impact (e.g. what an attacker can do, what data or systems are exposed)

You should receive an initial response as soon as possible. If the report is confirmed, we'll work with
you on a fix and coordinate a disclosure timeline before any public advisory is published. Reporters are
credited in the advisory unless they ask to remain anonymous.

## Scope

This policy covers the Bundesliga data-provider plugin: its WASM component build, its manifest
(declaring `api.openligadb.de` as its sole network host), and its mapping from `openligadb`'s
response types to the canonical `fulltime-plugin-api` schema.

Note what this crate explicitly does *not* enforce, since a report against these is really about
a different repo:

- The `data-provider` WIT interface, canonical schema, and manifest format themselves — that's
  [`fulltime-plugin-api`](https://github.com/pilgrimagesoftware/fulltime-plugin-api).
- The OpenLigaDB HTTP client and its request/response handling — that's
  [`openligadb`](https://github.com/pilgrimagesoftware/openligadb.rs).
- WASM sandboxing, network-host capability enforcement, resource limits, or fault isolation
  between plugins — that's the host runtime (`Apps/rust`).

Vulnerabilities in dependencies should be reported upstream; if a dependency issue affects this
plugin directly (e.g. no fix available, requires a workaround here), report it here as well.
