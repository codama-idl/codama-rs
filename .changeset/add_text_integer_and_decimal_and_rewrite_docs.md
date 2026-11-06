---
default: major
---

#### Add `Text`, `Integer` and `Decimal`, and rewrite `Docs` for spec v2

`Docs` is now an optional `Text` (a plain string or a `textNode`) and serialises as a single string. `Integer` and `Decimal` hold spec v2 numeric literals as canonical base-10 strings, with `Decimal::from(f64)` producing the same digits as JavaScript. `Node` gains `Transform` and `Text`, `TypeNode` gains `Integer`, `Float`, `FixedPoint` and `Duration`, and the v1 nesting machinery (`NestedTypeNode`, `#[nestable_type_node]`, the wrapper type nodes, the generic `NodeUnion` derive) is removed. The per-node constructors return over the following changes.
