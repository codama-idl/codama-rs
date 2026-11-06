---
default: major
---

#### Inject contextual defaults from `field(...)` and reject ambiguous fields

A contextual default such as `default_value = payer` written in a `field(...)` directive on an instruction is now injected through a `provides` entry, like a default written on a Rust field. Contextual defaults outside an instruction, two fields of a struct whose identifiers share the same camelCase form, and a `default_value` next to an `account(...)` directive are now errors instead of being dropped or accepted.
