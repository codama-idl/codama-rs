---
default: patch
---

#### Clarify that unresolved links only count as finite on their own

The docs of `get_has_finite_value_visitor` now state that a link that cannot be resolved counts as finite on its own, as its transforms are still checked, and its tests cover the prefixes of options, the sizes of enums and prefixed counts.
