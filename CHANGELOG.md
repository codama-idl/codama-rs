# Changelog

Every published crate of the workspace shares this changelog and the same version. Earlier releases are listed on the [GitHub releases](https://github.com/codama-idl/codama-rs/releases) page, under their `<crate>-vX.Y.Z` tags.

## 2.0.0-rc.0 (2026-11-13)

### Breaking Changes

#### Add `Text`, `Integer` and `Decimal`, and rewrite `Docs` for spec v2

`Docs` is now an optional `Text` (a plain string or a `textNode`) and serialises as a single string. `Integer` and `Decimal` hold spec v2 numeric literals as canonical base-10 strings, with `Decimal::from(f64)` producing the same digits as JavaScript. `Node` gains `Transform` and `Text`, `TypeNode` gains `Integer`, `Float`, `FixedPoint` and `Duration`, and the v1 nesting machinery (`NestedTypeNode`, `#[nestable_type_node]`, the wrapper type nodes, the generic `NodeUnion` derive) is removed. The per-node constructors return over the following changes.

#### Add `unit(...)` number displays

`display(unit("SOL"))`, or `display = unit(...)` inline on a number type, labels a number with a unit without scaling it, and the unit can also be a public key such as a mint address. Floats and fixed points take only this display, so an `amount(...)` display on them is now an error pointing to `unit(...)`, and an `amount(...)` display without `decimals` suggests either.

#### Build identifiers, paths, namespaces and decimals from string literals

`Identifier`, `Path`, `Namespace` and `Decimal` convert from string literals, so constructors take them directly, e.g. `AccountNode::new("my_account", data)` or `FieldDiscriminatorNode::new("header.kind", 0)`. A literal that breaks the spec grammar panics; parse any other string with `str::parse` or `TryFrom<String>`, which return the error instead.

#### Build spec v2 instruction data and defaults from Rust code

The fields of an instruction are now collected into a single `data` struct instead of a list of arguments, and a contextual default such as `#[codama(default_value = payer)]` is injected into its field from a `provides` entry on the instruction. Rust integer and float types map to `integerTypeNode` and `floatTypeNode`. Using `field(...)` on a tuple struct or variant is now an error, and an `enum_discriminator` name must be a valid identifier.

#### Emit transforms from wrapper attributes

`fixed_size(...)`, `size_prefix(...)`, `pre_offset(...)`, `post_offset(...)`, `sentinel(...)`, `hidden_prefix(...)` and `hidden_suffix(...)` keep their syntax, but they now attach transforms to the type they wrap, innermost first, rather than producing wrapper nodes in the IDL. They can also wrap a `link(...)`, and the prefix of `size_prefix(...)` must be an `integer(...)`.

#### Fail when modules or plugins describe the same item differently

When several modules describe the same program, two nodes of the same kind and identifier with different content, or two plugins with the same namespace and different payloads, are now an error instead of one silently replacing the other. Identical duplicates still merge into one. A linked PDA seed that matches no field is also an error.

#### Flatten `Node` into one variant per node kind

`Node` now has one variant per node kind instead of one per category, so `Node::Type(RegisteredTypeNode::StructField(field))` becomes `Node::StructFieldType(field)` and `Node::ContextualValue(RegisteredContextualValueNode::Pda(pda))` becomes `Node::PdaValue(pda)`. A new `NodeRef` enum borrows any node with the same variants, so a tree can be walked without cloning it. Every node and node union converts into `Node` and `NodeRef`, and back from `Node` with `try_from`. Deserialising a node of an unknown kind now names that kind in the error.

#### Inject contextual defaults from `field(...)` and reject ambiguous fields

A contextual default such as `default_value = payer` written in a `field(...)` directive on an instruction is now injected through a `provides` entry, like a default written on a Rust field. Contextual defaults outside an instruction, two fields of a struct whose identifiers share the same camelCase form, and a `default_value` next to an `account(...)` directive are now errors instead of being dropped or accepted.

#### Make `CodamaError` non-exhaustive

New error variants can now be added in minor releases, so a `match` on `CodamaError` outside this crate needs a wildcard arm.

#### Merge enum variant types and port value nodes to spec v2

The empty, tuple and struct enum variant types are replaced by a single `EnumVariantTypeNode` whose optional `data` can be any type node, and an `EnumValueNode` payload can now be any value node rather than an `EnumValuePayload`. Number values split into `IntegerValueNode` and `FloatValueNode`. Instruction arguments are referenced with a `DataValueNode` holding a validated `Path`, which replaces `ArgumentValueNode`, and `AccountDataValueNode` replaces `AccountFieldValueNode`.

#### Point attribute errors at their cause

Error messages now name the actual problem instead of a generic one: an out-of-range number in `default_value = ...` reports the range rather than `unrecognized value`, `scale` and `ticks_per_second` name their limit, a Rust name with no valid identifier explains why and suggests `identifier = "..."`, an invalid identifier states the identifier rule, and integer slots ask for an `integer(...)` type rather than an `IntegerTypeNode`. `data(name = "...")` points to `data("...")`, and conflicting plugins between modules name the module. Code matching on these messages needs updating.

#### Regenerate the nodes from spec v2

Node structs now follow spec v2: every node carries `plugins`, type nodes carry `transforms` in place of the v1 wrapper nodes, and `NumberTypeNode`/`NumberValueNode` are replaced by `IntegerTypeNode`, `FloatTypeNode`, `FixedPointTypeNode`, `DurationTypeNode`, `IntegerValueNode` and `FloatValueNode`. Named nodes serialise `identifier` instead of `name`, nodes are no longer `Copy`, and the v1-only nodes (`argumentValueNode`, `resolverValueNode`, `instructionArgumentNode`, the three enum variant nodes, `amountTypeNode`, `solAmountTypeNode`) are removed.

#### Reject PDA seeds that would otherwise be dropped

A seed linked to a field by name, such as `seed(name = "authority")`, now fails with an error asking for an explicit `type = ...` when the field's Rust type has no Codama type, instead of silently leaving the seed out of the PDA. A nested `data(...)` path in the seed list of a `pda(...)` value must also be named explicitly with `seed("name", data(...))`.

#### Remove `Docs::push`

`Docs` no longer has a `push` method. Build docs from a `Vec<String>` or a `Text` instead, e.g. `Docs::from(vec!["First line".to_string()])`.

#### Rename the `name` field of nodes to `identifier`

Following spec v2, the 22 nodes that identify themselves by a name now expose that name as `identifier` rather than `name`, and the `HasName` trait becomes `HasIdentifier` with an `identifier()` method. This only affects code constructing or reading nodes directly, e.g. plugins: the `#[codama(...)]` attributes and the generated IDL are unchanged by this step.

#### Replace `amount` and `sol_amount` with `fixed_point`, and add `duration` and ticks

`amount(number(u64), 6, "USDC")` becomes `fixed_point(integer(u64), scale = 6, unit = "USDC")`, with an optional `base`, and `sol_amount(...)` becomes `fixed_point(..., scale = 9, unit = "SOL")`. `integer(...)` and `float(...)` accept a `unit`, `date_time(...)` accepts `ticks_per_second`, and the new `duration(...)` type describes time spans. Using `amount(...)` or `sol_amount(...)` fails with a hint pointing to the replacement.

#### Replace wrapper type nodes with transforms

Layout modifiers no longer wrap the type they apply to: `FixedSizeTypeNode`, `SizePrefixTypeNode`, `PreOffsetTypeNode`, `PostOffsetTypeNode`, `SentinelTypeNode`, `HiddenPrefixTypeNode` and `HiddenSuffixTypeNode` become the `*TransformNode`s of spec v2, attached to any type node or defined type link with `add_transform`, e.g. `StringTypeNode::utf8().add_transform(FixedSizeTransformNode::new(42))`. Floats move to the new `FloatTypeNode`, `FixedPointTypeNode` and `DurationTypeNode` are added, and `DateTimeTypeNode` now wraps an `IntegerTypeNode`.

#### Simplify enum value constructors and directive checks

`EnumValueNode` now has two constructors: `new` builds a variant without data, and `with_value` builds a variant with any value as its data. The previous `empty`, `fields` and `tuple` constructors are removed. `Attributes::has_codama_attribute` is replaced by `Attributes::has`, which takes a directive filter such as `SkipDirective::filter` instead of a directive name. Invalid identifiers in attributes now point at the string itself. Variable seeds are now validated when the attribute is parsed, so `SeedDirectiveType::Variable` holds an `Identifier`.

#### Split `number` into `integer` and `float`, and replace `argument` with `field` and `data`

Write `integer(...)` or `float(...)` wherever you used `number(...)`, including in `size = ...` and `prefix = ...`. Instruction arguments are declared as data fields with `field(...)`, values that read from instruction data use `data(...)`, which also accepts nested paths such as `data("config.fees[0]")`, and `remaining_accounts(argument("x"), ...)` becomes `remaining_accounts("x", ...)`. Amount displays now require `decimals`, and enum variants accept any payload type. Using a removed spelling fails with a hint pointing to its replacement.

#### Start major 2

Release candidates of major 2 publish from `main` while it is in development. The stable release follows the ecosystem-wide [release process](https://github.com/codama-idl/spec/blob/HEAD/RELEASING.md).

#### Update count, discriminator and display node constructors for spec v2

`FieldDiscriminatorNode` now points at its field with a `Path`, validated against the spec path grammar like the `path` of `dataValueNode` and `accountDataValueNode`, so a nested field such as `header.kind` can serve as a discriminator. `AmountNumberDisplayNode::new` requires `decimals` and no longer implements `Default`, and `PrefixedCountNode::new` takes an `IntegerTypeNode`. New constructors are available for `SentinelCountNode`, `UnitNumberDisplayNode` and `ValueLabelDisplayNode`, along with `IntegerTypeNode::{le, be}` and `IntegerValueNode::new`.

#### Update program, account and instruction nodes for spec v2

Instructions describe their payload with a single optional `data` type node, plus an optional `return_data`, instead of a list of arguments. Accounts accept any type node as their data, remaining accounts are identified by name rather than carrying an argument or resolver value, and byte deltas use the spec v2 value nodes. Programs no longer have an `origin`, error and status messages accept rich text, and plugin namespaces are validated, so invalid ones are rejected when deserialising.

#### Use snake_case identifiers and replace `name = ...` with `identifier = ...`

Identifiers derived from Rust code are now snake_case (`MyAccount` → `my_account`, `HTTPServer` → `http_server`), whilst identifiers written in attributes, such as `field("x", ...)`, `pda("...")` or `identifier = "..."`, are kept exactly as written and must match the spec identifier grammar. References must therefore use the derived spelling, e.g. `pda("my_pda")` for `struct MyPda`. The `name = "..."` key becomes `identifier = "..."`, with a hint when the old key is used. In the Rust API, `CamelCaseString` is replaced by `Identifier`, which validates its value.

#### Validate paths, contextual values, interpolated intents and value labels

`Codama::get_idl` now also fails when a field discriminator path does not point to a struct field of the data it identifies, when a data, account or account data value does not resolve within its instruction, when an interpolated intent has an invalid or unresolvable placeholder, and when a value label is not a concrete value of the type of its field. `codama-visitors-core` gains `resolve_type_path` and `is_value_node_of_type`, and `codama-nodes` gains `Path::segments`, `parse_interpolated_intent` and `print_interpolated_intent`, like in the JavaScript library.

#### Validate the IDL with `codama-validators`

A new `codama-validators` crate lists the validation items of a Codama tree, like `@codama/validators` in the JavaScript library: missing identifiers, identifier collisions, links to missing nodes, programs without a public key, and more. `Codama::get_idl` and `Codama::get_node` now run it and fail with `FailedToValidateNode`, which lists every item, when any item is an error. In particular, a crate must now declare its program address, for example with `[package.metadata.solana] program-id = "..."`. The CLI prints the message of an error rather than its debug representation.

### Features

#### Add the `codama-visitors-core` crate

A new `codama-visitors-core` crate visits and transforms Codama nodes, like `@codama/visitors-core` in the JavaScript library. A `Visitor` returns a value for each node, with `IdentityVisitor`, `MergeVisitor` and `VoidVisitor` as bases and adapters such as `extend`, `tap`, `map`, `non_null` and `record_node_stack` to build on them. `NodeStack`, `NodePath` and `LinkableDictionary` track the visited path and resolve link nodes. `NodeRef` gains `children`, `map_children` and `identifier`, and converts back into a reference to its node with `try_from`.

#### Emit float values for numbers written at float positions

A whole number such as `default_value = 1` on a float field, constant, discriminator or PDA seed now produces a `floatValueNode` rather than an `integerValueNode`, following links to find the type, so the output matches spec v2 and `@codama/upgrade`.

#### List the expected node kinds in errors

Nodes and node unions now implement `NodeKinds`, whose `KINDS` constant lists the node kinds they can hold, like the `*_KINDS` constants of the JavaScript library. `ProvidedScope` uses it to name the expected kinds when a resolved value has the wrong kind, and reports a fallback of the wrong kind with a new `UnexpectedNodeKind` error.

#### Resolve injected values with `ProvidedScope`

`codama-visitors-core` gains `ProvidedScope`, which holds the `providedNode`s visible at a point of the tree and resolves the `injectedValueNode`s found there, like in the JavaScript library. The `record_provided_scope` adapter keeps it in sync with a visit, and `get_provided_scope_from_path` builds it from the instructions of a path.

### Fixes

#### Emit node attributes in spec declaration order

Nodes now serialise their attributes in the order the spec declares them, followed by `plugins`, which matches the IDLs produced by `@codama` JS. Only the order of keys in the JSON changes, not its content.

#### Keep defaults and plugins that were silently dropped

An instruction field renamed with `type = field("...", ...)` now keeps its `default_value`, and plugins set on a program by several modules are now merged instead of only keeping those of the first module.

## 1.0.0 (2026-11-05)

### Features

#### Promote the workspace to 1.0

The v1-era API is now stable: breaking changes land in major releases only, following the ecosystem-wide [release process](https://github.com/codama-idl/spec/blob/HEAD/RELEASING.md). This release is otherwise identical to 0.13.3.

## 0.13.3 (2026-11-04)

### Fixes

#### Accept the minimum value in signed integer attributes

Attributes reading a signed integer, such as `default_value` or the `offset` of `pre_offset` and `post_offset`, rejected the minimum value of the type (e.g. `-9223372036854775808` for an `i64`) because the sign was applied after parsing the digits. The whole range of the type is now accepted. (#142)

#### Identify `solana_program::pubkey::Pubkey` as a public key type

A field typed `solana_program::pubkey::Pubkey` was exported as a dangling `definedTypeLinkNode` named `pubkey` instead of a `publicKeyTypeNode`, like the `solana_sdk` and `solana_pubkey` paths already were. (#140)

#### Name the right union in node union deserialize errors

Failing to deserialize any node union reported `failed to deserialize AmountTypeNode`, whichever union was being deserialized. The message now names the actual union, e.g. `failed to deserialize TypeNode`. (#141)

## 0.13.2 (2026-08-24)

### Features

- Add the `remaining_accounts` directive for instructions (#137)
