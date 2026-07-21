# Contributing

## Development setup

This is a Rust crate targeting the WASM component model, with `openligadb` as its sole
non-plugin-api dependency. Clone the repo and build:

```bash
git clone https://github.com/pilgrimagesoftware/FullTime-Plugin-Bundesliga.git
cd FullTime-Plugin-Bundesliga
cargo build
```

The toolchain is pinned in `rust-toolchain.toml`; `rustup` picks it up automatically.

Before opening a PR, run what CI runs:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features --workspace
cargo test --no-default-features
cargo test --doc
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps
```

## Branching and PRs

This repo follows the standard `master`/`develop` git-flow:

- `master` reflects the latest released version; nothing is committed here directly.
- `develop` is the integration branch.
  Branch `feature/*` or `fix/*` off `develop`, and open your PR back into `develop`.
- `release/*` branches are cut automatically by the release workflow (see [RELEASING.md](RELEASING.md)).
  You shouldn't need to create one by hand.

CI (`ci.yaml`) runs `fmt`, `clippy`, `test`, and `doc` on every PR; all must pass before merging.

## Commit messages

Commits follow [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) (`feat:`, `fix:`, `chore:`, `docs:`, etc.).
This isn't enforced by CI today, but it's what `git-cliff` reads to generate the changelog and compute the next version on release.
A commit that doesn't follow the convention won't show up in the changelog and won't contribute to the version bump.

## Changing the mapping from openligadb to the canonical schema

`src/mapping.rs` is where `openligadb`'s response types (`Team`, `Match`, `TableTeam`, ...) get
mapped into `fulltime-plugin-api`'s canonical schema (`Competition`, `Fixture`, `Standings`, ...).
If `openligadb` changes a response shape, or `fulltime-plugin-api` changes the canonical schema,
update the mapping here rather than introducing a new intermediate type.

The contract itself (WIT interface, canonical schema, manifest format) lives in
[`fulltime-plugin-api`](https://github.com/pilgrimagesoftware/fulltime-plugin-api), not this
repo — see that repo's `docs/plugin-authoring.md` for the versioning policy this plugin must
stay compatible with.
