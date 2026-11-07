---
default: major
---

#### Use snake_case identifiers and replace `name = ...` with `identifier = ...`

Identifiers derived from Rust code are now snake_case (`MyAccount` → `my_account`, `HTTPServer` → `http_server`), whilst identifiers written in attributes, such as `field("x", ...)`, `pda("...")` or `identifier = "..."`, are kept exactly as written and must match the spec identifier grammar. References must therefore use the derived spelling, e.g. `pda("my_pda")` for `struct MyPda`. The `name = "..."` key becomes `identifier = "..."`, with a hint when the old key is used. In the Rust API, `CamelCaseString` is replaced by `Identifier`, which validates its value.
