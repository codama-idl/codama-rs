---
default: major
---

#### Merge enum variant types and port value nodes to spec v2

The empty, tuple and struct enum variant types are replaced by a single `EnumVariantTypeNode` whose optional `data` can be any type node, and an `EnumValueNode` payload can now be any value node rather than an `EnumValuePayload`. Number values split into `IntegerValueNode` and `FloatValueNode`. Instruction arguments are referenced with a `DataValueNode` holding a validated `Path`, which replaces `ArgumentValueNode`, and `AccountDataValueNode` replaces `AccountFieldValueNode`.
