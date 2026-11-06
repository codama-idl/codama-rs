---
default: major
---

#### Build spec v2 instruction data and defaults from Rust code

The fields of an instruction are now collected into a single `data` struct instead of a list of arguments, and a contextual default such as `#[codama(default_value = payer)]` is injected into its field from a `provides` entry on the instruction. Rust integer and float types map to `integerTypeNode` and `floatTypeNode`. Using `field(...)` on a tuple struct or variant is now an error, and an `enum_discriminator` name must be a valid identifier.
