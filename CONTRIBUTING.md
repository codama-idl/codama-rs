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

1. While change files are pending on `main`, every push refreshes a
   `Release package X.Y.Z` pull request with the version bump and the changelog
   entry they amount to.
2. Merging that pull request publishes every crate to crates.io with
   `cargo publish --workspace`, then tags the commit `vX.Y.Z` and creates the
   GitHub release. A failed publish is resumed by re-running the job: the
   workflow excludes the crates already on crates.io, which Cargo would
   otherwise refuse to publish again.

The workflow can also be dispatched by hand with its `dry-run` input to
rehearse either step without pushing or uploading anything.

This workspace is currently pre-1.0: breaking changes may land in any minor
release. Major releases follow the ecosystem-wide
[RELEASING.md](https://github.com/codama-idl/spec/blob/HEAD/RELEASING.md). The
plan is to promote the workspace to 1.0.0 on `main` when codama-rs joins the
spec v2 release train — freezing the v1-era API — and then cut the `1.x`
maintenance branch so that `main` hosts the v2 work (see the
[v2 tracking issue](https://github.com/codama-idl/spec/issues/102)).

