# Releasing

Releases are driven by [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) on `develop` and [`git-cliff`](https://git-cliff.org), via three workflows in `.github/workflows/` (thin wrappers around the `rust-*` reusable workflows in [`pilgrimagesoftware/github-actions`](https://github.com/pilgrimagesoftware/github-actions)).
There's no manual version bumping or changelog editing.

## 1. Prepare the release

Trigger **Prepare Release** manually (Actions tab → Prepare Release → Run workflow). It:

1. Runs `git-cliff --bumped-version` against `develop` to compute the next version from commits since the last tag (`fix:` → patch, `feat:` → minor, a `!` or `BREAKING CHANGE:` footer → major).
2. Bumps `Cargo.toml` to that version and prepends the generated section to `CHANGELOG.md`.
3. Pushes a `release/<version>` branch and opens (or updates, if one is already open) a PR from it into `master`, labeled `release` and requesting review.

If it fails with "nothing to release," either no `feat:`/`fix:`/etc. commits have landed on `develop` since the last tag, or no tag exists yet for `git-cliff` to compute a version relative to — see the workflow's error output for which.

## 2. Review and merge

Check the generated `CHANGELOG.md` section and the version bump on the PR.
Fix anything wrong by pushing to the `release/<version>` branch directly (or re-running Prepare Release, which force-pushes and updates the same PR).
Merge into `master` once CI passes.

## 3. Tag and publish

Merging the release PR triggers **Tag Release**, which tags `master` as `v<version>`.
The tag push triggers **Release**, which:

1. Re-runs the test suite against the tagged commit.
2. Publishes to crates.io.
3. Cuts a GitHub Release with that tag's changelog section as the release notes.
4. Merges `master` back into `develop` so the version bump and changelog aren't lost on the next release cycle.

## crates.io authentication

Publishing uses crates.io OIDC trusted publishing (`rust-lang/crates-io-auth-action`, via `release.yaml`'s `trusted-publishing: true` input) — no long-lived token in CI.
This requires bootstrapping the crate on crates.io with a one-time publish (`v0.1.0`) using a `CRATES_API_KEY` token, then configuring `fulltime-plugin-bundesliga` as a trusted publisher on crates.io before the first automated release runs.

## Manually re-running a release

If **Release** needs to be re-run for an existing tag (e.g. crates.io publish failed after the GitHub Release was already cut), trigger it manually with the `tag` input set to the existing `vX.Y.Z` tag rather than pushing a new tag.
