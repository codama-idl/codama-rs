# Migrating from 1.x to 2.0

Codama 2.0 generates IDLs that follow the [Codama spec v2](https://github.com/codama-idl/spec). Most programs only need the attribute renames below, and the compiler points to each one: a removed spelling fails with a hint naming its replacement.

```text
error: number(...) is removed in v2, use integer(...) or float(...)
 --> src/state.rs:4:21
  |
4 |     #[codama(type = number(u64))]
  |                     ^^^^^^^^^^^
```

The full list of changes lives in the [changelog](./CHANGELOG.md).

## Installing a release candidate

Cargo never picks a pre-release on its own, so a `"2"` requirement does not match `2.0.0-rc.0`. Ask for the candidate explicitly:

```toml
[dependencies]
codama = "=2.0.0-rc.0"
```

## Attributes

| 1.x                                              | 2.0                                                                          |
| ------------------------------------------------ | ---------------------------------------------------------------------------- |
| `name = "..."`                                   | `identifier = "..."`                                                         |
| `number(u64)`, `number(f32)`                     | `integer(u64)`, `float(f32)`                                                 |
| `amount(number(u64), 6, "USDC")`                 | `fixed_point(integer(u64), scale = 6, unit = "USDC")`                        |
| `sol_amount(number(u64))`                        | `fixed_point(integer(u64), scale = 9, unit = "SOL")`                         |
| `display(date_time)` on an integer               | `type = date_time(integer(i64))`                                             |
| `argument("amount", number(u64))` on instructions | `field("amount", integer(u64))`                                             |
| `default_value = argument("amount")`             | `default_value = data("amount")`, which also takes paths like `"config.fee"` |
| `remaining_accounts(argument("signers"))`        | `remaining_accounts("signers")`                                              |
| `discriminator(bytes = "...", encoding = "utf8")` | `discriminator(bytes = "...", encoding = utf8)`                             |

For example, an account and an instruction:

```rust
// 1.x
#[derive(CodamaAccount)]
#[codama(seed(type = string(utf8), value = "vault"))]
#[codama(seed(name = "authority", type = public_key))]
pub struct Vault {
    #[codama(type = public_key)]
    pub authority: [u8; 32],
    #[codama(type = amount(number(u64), 6, "USDC"))]
    pub balance: u64,
}

#[derive(CodamaInstruction)]
#[codama(account(name = "vault", writable))]
#[codama(account(name = "authority", signer))]
#[codama(argument("memo", string(utf8)))]
pub struct Deposit {
    #[codama(type = number(u64))]
    pub amount: u64,
}
```

```rust
// 2.0
#[derive(CodamaAccount)]
#[codama(seed(type = string(utf8), value = "vault"))]
#[codama(seed(identifier = "authority", type = public_key))]
pub struct Vault {
    #[codama(type = public_key)]
    pub authority: [u8; 32],
    #[codama(type = fixed_point(integer(u64), scale = 6, unit = "USDC"))]
    pub balance: u64,
}

#[derive(CodamaInstruction)]
#[codama(account(identifier = "vault", writable))]
#[codama(account(identifier = "authority", signer))]
#[codama(field("memo", string(utf8)))]
pub struct Deposit {
    #[codama(type = integer(u64))]
    pub amount: u64,
}
```

### Identifiers are snake_case

Identifiers derived from Rust names are now snake_case rather than camelCase, so a reference to another item uses that spelling. Identifiers you write yourself are kept exactly as written.

```rust
#[derive(CodamaPda)]
pub struct VaultPda; // identifier: "vault_pda" (1.x: "vaultPda")

#[codama(account(identifier = "vault", default_value = pda("vault_pda")))]
```

### The program address is required

`Codama::get_idl` now validates the IDL and fails on errors, such as a link pointing to a missing PDA or a program without an address. Declare the address of your program with `declare_id!` or in its manifest:

```toml
[package.metadata.solana]
program-id = "MyProgram1111111111111111111111111111111111"
```

## The generated IDL

The IDL follows spec v2, so it needs the 2.0 versions of the `@codama/*` JavaScript packages to render clients. In short:

- Nodes carry an `identifier` rather than a `name`, and the root declares `"version": "2.0.0"`.
- Instructions describe their payload with one `data` struct rather than a list of `arguments`.
- `numberTypeNode` splits into `integerTypeNode` and `floatTypeNode`, and `amountTypeNode` becomes `fixedPointTypeNode`.
- Wrappers such as `fixedSizeTypeNode` become `transforms` on the type they wrap.

`@codama/upgrade` turns a 1.x IDL into a 2.0 one, which helps compare the output of both versions.

## Plugins and the Rust API

### Depend on the crates you use

Plugins should depend on the crates whose types they use, such as `codama-nodes`, `codama-korok-visitors` and `codama-attributes`, rather than on `codama`. The `codama` crate is the entry point of programs: on Solana, it only exports the macros, and it does not re-export `codama-attributes` at all.

```toml
[dependencies]
codama-attributes = "=2.0.0-rc.0"
codama-korok-visitors = "=2.0.0-rc.0"
codama-nodes = "=2.0.0-rc.0"
codama-plugin-core = "=2.0.0-rc.0"
```

### Nodes

`Node` has one variant per node kind, nodes name themselves with `identifier`, and every union is `#[non_exhaustive]`:

```rust
// 1.x
let identifier: &str = match &node {
    Node::Type(RegisteredTypeNode::StructField(field)) => field.name.as_ref(),
    _ => return,
};

// 2.0
let identifier: &str = match &node {
    Node::StructFieldType(field) => field.identifier.as_ref(),
    _ => return,
};
```

- `HasName` becomes `HasIdentifier`, and `CamelCaseString` becomes `Identifier`, which validates its value.
- `NumberTypeNode` and `NumberValueNode` split into their integer and float counterparts.
- Wrapper type nodes become transforms: `StringTypeNode::utf8().add_transform(FixedSizeTransformNode::new(42))`.
- `NodeRef` borrows any node, so a tree can be walked without cloning it.
- `codama-visitors-core` and `codama-validators` are new: they visit, resolve and validate nodes like `@codama/visitors-core` and `@codama/validators`.

### Korok visitors

Each `KorokVisitor` method defaults to a matching `walk_*` function, like `syn::visit_mut`, which replaces `visit_children`:

```rust
// 1.x
fn visit_struct(&mut self, korok: &mut StructKorok) -> CodamaResult<()> {
    self.count += 1;
    self.visit_children(korok)
}

// 2.0
fn visit_struct(&mut self, korok: &mut StructKorok) -> CodamaResult<()> {
    self.count += 1;
    walk_struct(self, korok)
}
```

`CombineTypesVisitor` no longer exposes its fields: configure it with `new`, `strict`, `allowing_contextual_default_values` and `with_named_field`.

### Errors

`CodamaError` is `#[non_exhaustive]`, so a `match` on it needs a wildcard arm. `Filesystem` and `Manifest` errors now carry the `path` of the file involved, and validation failures surface as `FailedToValidateNode`, which lists every item.
