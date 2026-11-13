---
default: major
---

#### Validate paths, contextual values, interpolated intents and value labels

`Codama::get_idl` now also fails when a field discriminator path does not point to a struct field of the data it identifies, when a data, account or account data value does not resolve within its instruction, when an interpolated intent has an invalid or unresolvable placeholder, and when a value label is not a concrete value of the type of its field. `codama-visitors-core` gains `resolve_type_path` and `is_value_node_of_type`, and `codama-nodes` gains `Path::segments`, `parse_interpolated_intent` and `print_interpolated_intent`, like in the JavaScript library.
