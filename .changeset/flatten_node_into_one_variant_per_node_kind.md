---
default: major
---

#### Flatten `Node` into one variant per node kind

`Node` now has one variant per node kind instead of one per category, so `Node::Type(RegisteredTypeNode::StructField(field))` becomes `Node::StructFieldType(field)` and `Node::ContextualValue(RegisteredContextualValueNode::Pda(pda))` becomes `Node::PdaValue(pda)`. A new `NodeRef` enum borrows any node with the same variants, so a tree can be walked without cloning it. Every node and node union converts into `Node` and `NodeRef`, and back from `Node` with `try_from`. Deserialising a node of an unknown kind now names that kind in the error.
