# codama-cli

Command-line interface for generating [Codama](https://github.com/codama-idl/codama-rs) IDL from Rust projects.

## Installation

```bash
cargo install --path codama-cli
```

Or build from source:

```bash
cargo build --release -p codama-cli
```

## Usage

### Generate IDL to stdout

```bash
codama-cli generate ./my-program
```

### Pretty-print the output

```bash
codama-cli generate ./my-program --pretty
```

### Save to file

```bash
codama-cli generate ./my-program -o idl.json
```

### Combine flags

```bash
codama-cli generate ./my-program --pretty --output idl.json
```

## Command Reference

```
Command-line interface for Codama

Usage: codama-cli <COMMAND>

Commands:
  generate  Generate IDL from a Rust project
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### `generate` subcommand

```
Generate IDL from a Rust project

Usage: codama-cli generate [OPTIONS] [PATH]

Arguments:
  [PATH]  Path to the Rust project (defaults to current directory) [default: .]

Options:
  -o, --output <OUTPUT>  Output file path (defaults to stdout)
  -p, --pretty           Pretty-print the JSON output
  -h, --help             Print help
```

## Examples

### Example 1: Generate from current directory

```bash
cd my-solana-program
codama-cli generate
```

### Example 2: Generate with formatted JSON

```bash
codama-cli generate ./programs/my-program --pretty
```

Output:
```json
{
  "kind": "rootNode",
  "standard": "codama",
  "version": "0.6.3",
  "program": {
    "kind": "programNode",
    "name": "myProgram",
    ...
  }
}
```

### Example 3: Save to file for processing

```bash
codama-cli generate ./my-program -o idl.json
# IDL written to: idl.json
```

## Requirements

- Rust toolchain
- A Rust project with a valid `Cargo.toml` and library or binary target

## License

Apache-2.0
