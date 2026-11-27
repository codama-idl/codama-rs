---
default: major
---

#### Turn the variants of accounts, events and instructions into items the same way

The visitors turning the variants of a `CodamaAccounts`, `CodamaEvents` or `CodamaInstructions` enum into program items share one way of numbering variants and reading their data, so tuple variants are accepted everywhere, including for accounts, and a `default_value` on a tuple item reaches the field it becomes, as it already did for instructions. A discriminator that would overflow a `u32` is an error. Duplicate field identifiers are reported by the validator, once against their path, rather than also by the visitors; `CombineTypesVisitor` configures through `allowing_contextual_default_values` and `with_named_field`, and its `r#override` and `parent_enum` fields are no longer public.
