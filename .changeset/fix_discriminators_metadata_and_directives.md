---
default: major
---

#### Number enum variants like Rust and apply directives where they were ignored

- The variants following an explicit discriminant carry the discriminator Rust gives them, e.g. `Frozen` in `enum Status { Active = 5, Frozen }` is `6` rather than its index. A discriminant that is not an unsigned integer literal fitting a `u32` is an error instead of being ignored.
- The `declare_id!` and `codama_program!` macros of a crate no longer leak into the other crates of a `Codama::load_all`, and `declare_id!` given a constant no longer sets the public key to the name of that constant.
- A `program(...)` directive applies on a `mod file;` declaration, at the top of a file (`#![codama(program(...))]`) and at the top of the crate root, where the declaration wins over the file. One on a variant or field is an error.
- Type modifiers such as `fixed_size` apply to type aliases, and an exported type alias that does not resolve to a type is an error.
- An item deriving several Codama items, such as `#[derive(CodamaType, CodamaAccount)]`, is an error, as the derives would conflict. `Attributes::has_any_codama_derive` is replaced by `Attributes::codama_derives`.
