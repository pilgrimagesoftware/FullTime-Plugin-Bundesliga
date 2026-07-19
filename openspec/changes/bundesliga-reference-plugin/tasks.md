## 1. Crate Setup

- [ ] 1.1 Scaffold the WASM component crate (Cargo.toml targeting `wasm32-wasip2` or the
  Component Model target used by `fulltime-plugin-api`, license, CI)
- [ ] 1.2 Add dependencies on `openligadb` (pinned version) and `fulltime-plugin-api`

## 2. Transport Shim

- [ ] 2.1 Implement request construction matching `openligadb`'s existing HTTP calls
- [ ] 2.2 Route requests through the host `fetch` capability instead of a direct HTTP
  client
- [ ] 2.3 Feed responses back into `openligadb`'s existing deserialization/model types
- [ ] 2.4 Add integration tests for the shim using recorded/fixture HTTP responses (no live
  calls in tests)

## 3. Schema Mapping

- [ ] 3.1 Implement mapping from `openligadb` league/team types to canonical
  competition/team types
- [ ] 3.2 Implement mapping from `openligadb` match types to canonical fixture/result types
- [ ] 3.3 Implement mapping from `openligadb` table types to canonical standings
- [ ] 3.4 Return `schema-mapping-failure` for any upstream shape that can't be represented
  in the canonical schema, rather than partial data

## 4. Plugin Interface Implementation

- [ ] 4.1 Implement the data-provider WIT interface (list-competitions, fetch-fixtures,
  fetch-results, fetch-standings, fetch-metadata) against the shim and mapping layers
- [ ] 4.2 Write the plugin manifest declaring the OpenLigaDB API host as the sole network
  capability and the targeted schema/interface versions

## 5. Validation

- [ ] 5.1 Run this plugin's path and a direct `openligadb` call side by side for a sample
  of seasons/matchdays, diffing output against the canonical schema
- [ ] 5.2 Document any fields the canonical schema doesn't carry that `openligadb`
  provides, and confirm with the `fulltime-plugin-api` change owner whether the schema
  needs to account for them
