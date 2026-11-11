---
default: major
---

#### Validate the IDL with `codama-validators`

A new `codama-validators` crate lists the validation items of a Codama tree, like `@codama/validators` in the JavaScript library: missing identifiers, identifier collisions, links to missing nodes, programs without a public key, and more. `Codama::get_idl` and `Codama::get_node` now run it and fail with `FailedToValidateNode`, which lists every item, when any item is an error. In particular, a crate must now declare its program address, for example with `[package.metadata.solana] program-id = "..."`. The CLI prints the message of an error rather than its debug representation.
