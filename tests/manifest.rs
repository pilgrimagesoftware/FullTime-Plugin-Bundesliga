//! Validates this plugin's manifest against `fulltime-plugin-api`'s parser, per
//! `openspec/changes/bundesliga-reference-plugin/tasks.md` (4.2).

use fulltime_plugin_api::{Manifest, INTERFACE_VERSION, SCHEMA_VERSION};

#[test]
fn manifest_parses_and_declares_only_the_openligadb_host() {
    let source = include_str!("../manifest.toml");

    let manifest = Manifest::parse(source).unwrap();

    assert_eq!(manifest.id, "bundesliga");
    assert_eq!(manifest.network_hosts, ["api.openligadb.de"]);
    assert!(SCHEMA_VERSION.accepts(manifest.schema_version));
    assert!(INTERFACE_VERSION.accepts(manifest.interface_version));
}
