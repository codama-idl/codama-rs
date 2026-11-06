---
default: patch
---

#### Emit node attributes in spec declaration order

Nodes now serialise their attributes in the order the spec declares them, followed by `plugins`, which matches the IDLs produced by `@codama` JS. Only the order of keys in the JSON changes, not its content.
