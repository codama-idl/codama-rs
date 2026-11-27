# Codama for Rust

Codama for Rust describes your Solana programs with Rust attributes and generates a [Codama](https://github.com/codama-idl/codama) IDL from their source code. The IDL then renders clients, documentation and more through the Codama ecosystem.

```rust
use codama::{codama, CodamaAccount, CodamaInstruction};

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
pub struct Deposit {
    pub amount: u64,
}
```

The attributes compile to nothing, including on Solana, so they add no weight to your program.

## Generating the IDL

Install the CLI and point it at your program's crate:

```sh
cargo install codama-cli
codama-rs generate-idl ./programs/my-program --pretty -o idl.json
```

Or generate it from Rust, for example in a script or a `build.rs`:

```rust
let idl = codama::Codama::load("programs/my-program")?.get_json_idl()?;
```

Your program needs an address, either from `declare_id!` or from its manifest:

```toml
[package.metadata.solana]
program-id = "MyProgram1111111111111111111111111111111111"
```

## Crates

Programs only need `codama`. The other crates are for plugins and tools building on Codama, which should depend on the crates they use directly.

| Crate                   | Purpose                                                                                     |
| ----------------------- | ------------------------------------------------------------------------------------------- |
| `codama`                | The entry point: the attribute macros and, outside Solana, the `Codama` API that builds IDLs |
| `codama-cli`            | The `codama-rs` command-line interface                                                      |
| `codama-macros`         | The `#[codama(...)]` attribute and the `Codama*` derives                                    |
| `codama-attributes`     | Parses Codama attributes into directives                                                    |
| `codama-nodes`          | The nodes of the Codama IDL                                                                 |
| `codama-visitors-core`  | Visits, transforms and resolves nodes, like `@codama/visitors-core`                         |
| `codama-validators`     | Validates nodes, like `@codama/validators`                                                  |
| `codama-stores`         | Loads crates and their files                                                                |
| `codama-koroks`         | A tree of the items of your crates, each with the node it describes                         |
| `codama-korok-visitors` | Visitors that fill and combine the nodes of a korok tree                                    |
| `codama-plugin-core`    | The `KorokPlugin` trait and the default plugin                                              |
| `codama-errors`         | The error and result types shared by every crate                                            |
| `codama-syn-helpers`    | Extensions of `syn` used to parse attributes                                                |

## Upgrading from 1.x

Version 2.0 follows the Codama spec v2. The [migration guide](https://github.com/codama-idl/codama-rs/blob/main/MIGRATION.md) covers the attribute renames and the API changes, and the compiler points to every removed spelling.

## Contributing

See [CONTRIBUTING.md](https://github.com/codama-idl/codama-rs/blob/main/CONTRIBUTING.md).
