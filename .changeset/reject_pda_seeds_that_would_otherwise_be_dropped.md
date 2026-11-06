---
default: major
---

#### Reject PDA seeds that would otherwise be dropped

A seed linked to a field by name, such as `seed(name = "authority")`, now fails with an error asking for an explicit `type = ...` when the field's Rust type has no Codama type, instead of silently leaving the seed out of the PDA. A nested `data(...)` path in the seed list of a `pda(...)` value must also be named explicitly with `seed("name", data(...))`.
