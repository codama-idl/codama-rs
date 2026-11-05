# Contributing

Thanks for your interest in contributing to Codama's Rust implementation.

## Getting set up

You'll need a recent Rust toolchain (CI pins the exact version via
`RUST_TOOLCHAIN` in `.github/workflows/main.yml`) plus Node.js and pnpm for
the JS-based code generator. To match CI:

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --all-features
cargo test --all-features
pnpm install --frozen-lockfile && pnpm lint && pnpm test
```

CI also verifies that generated code is up to date, so regenerate and commit
any generator-driven output alongside your changes.

## Documenting changes

Every published crate of the workspace shares one version and one
`CHANGELOG.md`. A pull request that changes a published crate documents that
change with a change file in `.changeset/`, which [Knope](https://knope.tech)
turns into the next version bump and changelog entry at release time. Run
`knope document-change` from the repository root, or write the file by hand:

```markdown
---
default: minor
---

#### Add the `remaining_accounts` directive

Instructions can now declare remaining accounts with `#[codama(remaining_accounts(...))]`.
```

- The front matter sets the bump of the whole workspace: `patch`, `minor` or
  `major`.
- The body is the changelog entry. Start it with a `####` heading and describe
  the change from the point of view of a user of the crates.
- A pull request that does not warrant a release note, e.g. a CI or
  documentation change, carries the `no changelog` label instead.

The Knope Bot check on each pull request fails until it has a change file or
the label, and offers maintainers a button to add a change file.

## Releasing

Releases are automated by the `Release` workflow
(`.github/workflows/release.yml`), which mirrors the changesets flow of the
JavaScript repositories:

1. While change files are pending on a release branch (`main` or a `N.x`
   maintenance branch), every push refreshes a `Release package X.Y.Z` pull
   request against it with the version bump and the changelog entry they
   amount to.
2. Merging that pull request publishes every crate to crates.io with
   `cargo publish --workspace`, then tags the commit `vX.Y.Z` and creates the
   GitHub release. A failed publish is resumed by re-running the job: the
   workflow excludes the crates already on crates.io, which Cargo would
   otherwise refuse to publish again.

The workflow can also be dispatched by hand: its `dry-run` input rehearses
either step without pushing or uploading anything, and its `override-version`
input releases a version other than the one the change files amount to, for the
cases Knope cannot compute itself, such as the first stable major from a `0.x`
version. The override needs pending change files to apply to, and only lasts
until the next push to the branch refreshes the release pull request: dispatch
it last, right before merging that pull request.

### Major releases

Major releases follow the ecosystem-wide
[RELEASING.md](https://github.com/codama-idl/spec/blob/HEAD/RELEASING.md):
`main` always hosts the bleeding edge, each released major N keeps a `N.x`
maintenance branch, and a new major goes through a cut and a promote. Both are
`workflow_dispatch` workflows that commit as the release app:

- `Cut` (`.github/workflows/cut.yml`), with `main` at major N: creates `N.x`
  from the tip of `main` (its birth commit points `knope.toml` at `N.x` for its
  release pull requests), puts `main` in pre-release mode by setting
  `prerelease_label = "rc"` on the `PrepareRelease` step of `knope.toml`,
  seeds the change file of the major bump, and makes `N.x` the default branch.
  From then on each merged release pull request on `main` publishes a
  candidate of the next major: `(N+1).0.0-rc.0`, `(N+1).0.0-rc.1`, and so on.
  In pre-release mode the change files survive the candidates, so every
  candidate's changelog entry restates the whole major.
- `Promote` (`.github/workflows/promote.yml`), once the candidacy is complete:
  removes the prerelease label from `knope.toml` on `main`, which makes the
  release workflow open the pull request of the stable `(N+1).0.0`, consuming
  the accumulated change files, and makes `main` the default branch again.

Both workflows skip the steps a previous run completed, so a run that failed
midway is resumed by dispatching it again. They always operate on `main`, but
GitHub runs the workflow file of the branch selected at dispatch time, which
defaults to the default branch: while `N.x` is the default branch, dispatch
`Promote` with `main` selected to pick up any fix to the workflow landed there.

crates.io has no distribution tags, so the maintenance branch needs no change at
promote time: Cargo never resolves a `N` requirement to the next major by
itself. A maintenance release is an ordinary pull request against `N.x` with a
change file, followed by the release pull request that opens against `N.x`.

This workspace is currently pre-1.0: breaking changes may land in any minor
release. The plan is to promote the workspace to 1.0.0 on `main` when codama-rs
joins the spec v2 release train — freezing the v1-era API — and then cut the
`1.x` maintenance branch so that `main` hosts the v2 work (see the
[v2 tracking issue](https://github.com/codama-idl/spec/issues/102)). That first
stable release goes through the `override-version` input of the `Release`
workflow, since `Cut` only runs from 1.0.0 onwards.
