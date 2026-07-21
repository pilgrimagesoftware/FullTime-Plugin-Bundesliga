## ADDED Requirements

### Requirement: Implements the Data-Provider Interface
This plugin SHALL implement `fulltime-plugin-api`'s data-provider WIT interface: list
competitions, fetch fixtures, fetch results, fetch standings, and fetch metadata, for the
Bundesliga.

#### Scenario: Host calls fetch-fixtures
- **WHEN** the host invokes `fetch-fixtures` on this plugin for a given Bundesliga season
  and matchday
- **THEN** the plugin returns fixture data conforming to the canonical `league-data-schema`

### Requirement: Requests Go Through the Host Fetch Capability
This plugin SHALL NOT open any direct network connection; all upstream OpenLigaDB API
calls SHALL go through the host-provided `fetch` capability.

#### Scenario: Plugin fetches Bundesliga data
- **WHEN** this plugin needs data from the OpenLigaDB API
- **THEN** it issues the request via the host's `fetch` import rather than any direct
  socket or HTTP client call

### Requirement: Canonical Schema Mapping
This plugin SHALL map every `openligadb` response type it consumes to
`fulltime-plugin-api`'s canonical schema before returning data to the host.

#### Scenario: League table is mapped to canonical standings
- **WHEN** this plugin fetches a Bundesliga table via `openligadb`'s response types
- **THEN** the returned standings conform to the canonical schema as a single ranked
  table

#### Scenario: Upstream data cannot be mapped
- **WHEN** `openligadb` returns a response this plugin cannot map to the canonical schema
- **THEN** the plugin returns the `schema-mapping-failure` error variant rather than
  partial or malformed schema data

### Requirement: Manifest Declares OpenLigaDB as the Sole Network Capability
This plugin's manifest SHALL declare exactly the OpenLigaDB API host as its required
network capability, and no other host.

#### Scenario: Host loads this plugin
- **WHEN** the host parses this plugin's manifest
- **THEN** the declared network hosts list contains only the OpenLigaDB API host

### Requirement: Output Equivalence with Direct Integration
This plugin's output SHALL be verifiably equivalent to `openligadb`'s current
direct-integration output for the same query, when both are mapped to the canonical
schema.

#### Scenario: Side-by-side comparison for a given season/matchday
- **WHEN** this plugin and a direct `openligadb` call are both queried for the same
  Bundesliga season and matchday
- **THEN** the canonical-schema representations of both results match, aside from fields
  the canonical schema doesn't carry (which are itemized in `design.md` if any exist)
