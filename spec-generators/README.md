# `@codama-internal/rust-spec-generators`

Private project. Houses the code generator that turns the `@codama/spec` encoded specification into the source code of the Codama Rust monorepo crates.

This project is **never published**. It is only invoked at development time, via `pnpm generate` from this directory, to regenerate specific subtrees of the Rust monorepo (today: `codama-nodes/src/generated/`).

## Architecture

The generator owns its full pipeline. It knows which spec major it targets, which output directory it writes to, and which compatibility knobs it needs. It does not take a spec as a parameter — it imports it directly from `@codama/spec`.

The bin script (`bin/generate.ts`) wraps the top-level `generate()` function for `pnpm generate`.

## Layout

```
src/
├── index.ts            # generate() + getRenderMap() (pure)
├── options.ts          # RenderOptions, buildRenderScope, validateRenderOptions
├── defaults.ts         # CATEGORY_DIRECTORIES, name overrides, CATEGORY_ROUTING
├── repoDirectory.ts    # locates the Rust workspace root
├── unions.ts           # flattenNodeUnion + getEmittableUnions
└── fragments/
    ├── helpers.ts          # `use(name)` — Rust analogue of the JS `use(…)` helper
    ├── typeExpr.ts         # spec TypeExpr → Rust Fragment
    ├── attributeBodyLine.ts
    ├── structFragment.ts
    ├── fromImpl.ts
    ├── hasNameImpl.ts
    ├── nodePage.ts
    ├── unionPage.ts
    ├── page.ts             # use-block renderer (groups crate::*)
    └── modPage.ts          # Rust mod.rs renderer

bin/generate.ts         # pnpm generate entry; wraps generate()

test/
├── fragments/          # one .test.ts per fragment renderer
├── generate.test.ts    # validateRenderOptions + render-map assertions
└── scope.test.ts       # buildRenderScope tests
```

## RenderMap pipeline

The generator builds an in-memory `RenderMap<Fragment>` — a map from output path to a fragment carrying content + an `ImportMap` — before touching the filesystem.

Per spec entity, the generator emits a page fragment composed from focused sub-fragments (`structFragment`, `fromImpl`, `hasNameImpl`, `unionPage`, …). Each sub-fragment attaches its referenced identifiers as concrete `crate::Foo` imports via the shared `use(name)` helper in `fragments/helpers.ts`.

The page renderer (`fragments/page.ts`) groups every `use crate::Foo;` line into a single `use crate::{…};` block, keeps non-`crate::` paths (e.g. `codama_nodes_derive::node`) on their own lines, and prepends the import block to the page body.

The mod-page renderer (`fragments/modPage.ts`) walks every folder in the spec pages, emits a `mod.rs` listing the per-node files, and a root `mod.rs` listing the subdirectories.

Finally, `generate()` calls `writeRenderMap` (from `@codama/fragments`) to flush the final map to disk, wiping the target directory first so stale files cannot survive.

## Running

```sh
pnpm install
pnpm generate
```

`pnpm generate` builds the orchestrator with `tsup`, runs the bundled script via `node ./dist/generate.mjs`, and then runs `cargo fmt -p codama-nodes` to keep the output rustfmt-clean. A CI job (`spec-generators` in `.github/workflows/main.yml`) runs the same pipeline and fails on `git diff --exit-code`, catching anyone who forgets to regenerate after editing the spec dep.

## Scope (v1)

This first iteration generates source files for nodes and unions in spec category `link`. Each generated per-node file contains the struct definition wrapped in `#[node]`, a `From<Self> for crate::Node` impl routing through the category's union variant, and a `HasName` impl when the node has a `name: stringIdentifier()` attribute. Each generated per-union file contains the union enum wrapped in `#[node_union]` plus a `HasName` impl when every member node has a `name` attribute.

Other categories (`count`, `discriminator`, `pdaSeed`, `value`, `topLevel`, `contextualValue`, `type`) stay hand-written for now and will land in future PRs. Per-node constructors (`AccountLinkNode::new`, etc.), bespoke `TryFrom` impls, and `#[cfg(test)] mod tests` blocks also stay hand-written, in `codama-nodes/src/<name>.rs` files that `use` the generated struct.

Nested-union type aliases, enumerations, and the top-level `Node` registry stay hand-written. Expanding the generator's scope to cover them is a future PR.

## Bumping `@codama/spec`

1. Update the version pin in `package.json`.
2. `pnpm install`.
3. `pnpm generate`.
4. Review the diff under `codama-nodes/src/generated/`, run `cargo test --workspace`, fix any consumer fallout in the hand-written `codama-nodes/src/<name>.rs` files.

## Tests

Tests mirror the source layout: one `test/fragments/<name>.test.ts` per fragment renderer, plus a `test/scope.test.ts` for `buildRenderScope` and a `test/generate.test.ts` for option validation + render-map assertions via `getFromRenderMap` (no filesystem access).
