---
default: major
---

#### Number enum variants like Rust and apply directives where they were ignored

The variants following an explicit discriminant now carry the discriminator Rust gives them, e.g. `Frozen` in `enum Status { Active = 5, Frozen }` is `6` rather than its index, and a discriminant that is not an unsigned integer literal is an error instead of being ignored. The `declare_id!` and `codama_program!` macros of a crate no longer leak into the other crates of a `Codama::load_all`. A `program(...)` directive on a `mod file;` declaration or at the top of its file (`#![codama(program(...))]`) now applies, and one on a variant or field is an error. Type modifiers such as `fixed_size` now apply to type aliases, and an exported type alias that does not resolve to a type is an error. An item deriving several Codama items, such as `#[derive(CodamaType, CodamaAccount)]`, is an error, as the derives would conflict.
