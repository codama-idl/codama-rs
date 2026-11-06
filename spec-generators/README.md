# `@codama-internal/rust-spec-generators`

Private project. Houses the code generator that turns the `@codama/spec` encoded specification into the source code of the Codama Rust monorepo crates.

This project is **never published**. It is only invoked at development time, via `pnpm generate` from this directory, to regenerate specific subtrees of the Rust monorepo (today: `codama-nodes/src/generated/`).

## Architecture

The generator owns its full pipeline. It knows which spec major it targets, which output directory it writes to, and which compatibility settings it needs. It does not take a spec as a parameter — it imports it directly from `@codama/spec`.

The bin script (`bin/generate.ts`) wraps the top-level `generate()` function for `pnpm generate`.

## Layout

```
src/
├── index.ts            # generate() + getRenderMap() (pure)
├── options.ts          # RenderOptions, buildRenderScope, validateRenderOptions
├── defaults.ts         # CATEGORY_DIRECTORIES, name overrides, CATEGORY_ROUTING
├── repoDirectory.ts    # locates the Rust workspace root
├── unions.ts           # flattenNodeUnion + getEmittableUnions
├── baseAttributes.ts   # withBaseAttributes: appends the spec base attributes (plugins) to every node
├── nodeFlavor.ts       # getNodeMacroFlavor: #[node] or #[type_node] per node
├── literalUnions.ts    # getReferencedLiteralUnions + literalUnionVariantName
└── fragments/
    ├── index.ts            # re-exports every fragment renderer
    ├── helpers.ts          # `use(name)` — Rust analogue of the JS `use(…)` helper
    ├── typeExpr.ts         # spec TypeExpr → Rust Fragment
    ├── attributeBodyLine.ts
    ├── nodeStructFragment.ts
    ├── fromImpl.ts
    ├── hasIdentifierImpl.ts
    ├── nodePage.ts
    ├── unionPage.ts
    ├── registeredUnionPage.ts  # category union with `#[registered]`-only variants
    ├── enumPage.ts         # spec enumeration -> Rust enum shell
    ├── literalUnionPage.ts # literalUnion -> Rust enum shell
    ├── codamaVersionPage.ts    # CODAMA_VERSION constant
    ├── page.ts             # use-block renderer (groups crate::*)
    └── modPage.ts          # Rust mod.rs renderer

bin/generate.ts         # pnpm generate entry; wraps generate()

test/
├── fragments/          # one .test.ts per fragment renderer
├── generate.test.ts    # validateRenderOptions + render-map assertions
├── baseAttributes.test.ts
├── literalUnions.test.ts
├── nodeFlavor.test.ts
├── unions.test.ts
└── scope.test.ts       # buildRenderScope tests
```

## RenderMap pipeline

The generator builds an in-memory `RenderMap<Fragment>` — a map from output path to a fragment carrying content + an `ImportMap` — before touching the filesystem.

Per spec entity, the generator emits a page fragment composed from focused sub-fragments (`nodeStructFragment`, `fromImpl`, `hasIdentifierImpl`, `unionPage`, …). Each sub-fragment attaches its referenced identifiers as concrete `crate::Foo` imports via the shared `use(name)` helper in `fragments/helpers.ts`.

The page renderer (`fragments/page.ts`) groups every `use crate::Foo;` line into a single `use crate::{…};` block, keeps non-`crate::` paths (e.g. `codama_nodes_derive::node`) on their own lines, and prepends the import block to the page body.

The mod-page renderer (`fragments/modPage.ts`) walks every folder in the spec pages, emits a `mod.rs` listing the per-node files, and a root `mod.rs` listing the subdirectories.

Finally, `generate()` calls `writeRenderMap` (from `@codama/fragments`) to flush the final map to disk, wiping the target directory first so stale files cannot survive.

## Running

```sh
pnpm install
pnpm generate
```

`pnpm generate` builds the orchestrator with `tsup`, runs the bundled script via `node ./dist/generate.mjs`, and then runs `cargo fmt -p codama-nodes` to keep the output rustfmt-clean. A CI job (`spec-generators` in `.github/workflows/main.yml`) runs the same pipeline and fails on `git diff --exit-code`, catching anyone who forgets to regenerate after editing the spec dep.

## Scope

The generator emits a source file for every node of the spec categories in `CATEGORY_ROUTING` (`src/defaults.ts`), plus their unions, the `shared` enumerations and literal unions, and `CODAMA_VERSION`. Each generated per-node file contains the struct definition wrapped in `#[node]` or `#[type_node]`, a `From<Self> for crate::Node` impl routing through the category's union variant (except `topLevel` nodes, which are direct `Node` variants), and a `HasIdentifier` impl when the node has a required `identifier: stringIdentifier()` attribute. Each generated per-union file contains the union enum wrapped in `#[node_union]` plus a `HasIdentifier` impl when every member node has one.

Per-node constructors (`AccountLinkNode::new`, etc.), bespoke `TryFrom` impls, and `#[cfg(test)] mod tests` blocks stay hand-written. They live in `codama-nodes/src/`, at the path of the generated file without `generated/` (for example `codama-nodes/src/link_nodes/account_link_node.rs`), and `use` the generated struct.

The `type` category unions, the top-level `Node` registry, and the bespoke impls of enumerations and literal unions (`Default`, custom serde) stay hand-written.

## Bumping `@codama/spec`

1. Update the version pin in `package.json`.
2. `pnpm install`.
3. `pnpm generate`.
4. Review the diff under `codama-nodes/src/generated/`, run `cargo test --workspace`, fix any consumer fallout in the hand-written files under `codama-nodes/src/`.

## Tests

Tests mirror the source layout: `test/fragments/<name>.test.ts` for the fragment renderers and `test/<name>.test.ts` for the top-level modules, plus a `test/scope.test.ts` for `buildRenderScope` and a `test/generate.test.ts` for option validation + render-map assertions via `getFromRenderMap` (no filesystem access). `generate.test.ts` also covers the renderers without their own test file (`codamaVersionPage`, `registeredUnionPage`).
