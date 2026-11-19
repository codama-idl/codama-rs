---
default: major
---

#### Validate cyclic defined types without a finite value

`Codama::get_idl` now fails when a defined type links back to itself such that every value nests another one, e.g. `loop = struct { next: link(loop) }` or `alias = link(alias)`, since no codec can encode or decode it. Cycles with a way out, such as an option, an enum with another variant or a collection that may be empty, are still accepted. `codama-visitors-core` gains `is_defined_type_cyclic`, `has_defined_type_finite_value`, `get_defined_type_links_visitor` and `get_has_finite_value_visitor`, like in the JavaScript library.
