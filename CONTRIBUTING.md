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

## Releasing

Crates are published to crates.io manually, via the `Publish Crates` workflow
(`.github/workflows/publish.yml`): a `workflow_dispatch` per crate, driven by
`cargo-release`, with a dry-run option.

This workspace is currently pre-1.0: breaking changes may land in any minor
release. Major releases follow the ecosystem-wide
[RELEASING.md](https://github.com/codama-idl/spec/blob/HEAD/RELEASING.md). The
plan is to promote the workspace to 1.0.0 on `main` when codama-rs joins the
spec v2 release train — freezing the v1-era API — and then cut the `1.x`
maintenance branch so that `main` hosts the v2 work (see the
[v2 tracking issue](https://github.com/codama-idl/spec/issues/102)).
