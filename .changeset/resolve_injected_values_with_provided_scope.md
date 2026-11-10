---
default: minor
---

#### Resolve injected values with `ProvidedScope`

`codama-visitors-core` gains `ProvidedScope`, which holds the `providedNode`s visible at a point of the tree and resolves the `injectedValueNode`s found there, like in the JavaScript library. The `record_provided_scope` adapter keeps it in sync with a visit, and `get_provided_scope_from_path` builds it from the instructions of a path.
