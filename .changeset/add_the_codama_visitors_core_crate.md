---
default: minor
---

#### Add the `codama-visitors-core` crate

A new `codama-visitors-core` crate visits and transforms Codama nodes, like `@codama/visitors-core` in the JavaScript library. A `Visitor` returns a value for each node, with `IdentityVisitor`, `MergeVisitor` and `VoidVisitor` as bases and adapters such as `extend`, `tap`, `map`, `non_null` and `record_node_stack` to build on them. `NodeStack`, `NodePath` and `LinkableDictionary` track the visited path and resolve link nodes. `NodeRef` gains `children`, `map_children` and `identifier`, and converts back into a reference to its node with `try_from`.
