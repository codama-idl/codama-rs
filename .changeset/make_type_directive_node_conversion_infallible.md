---
default: major
---

#### Make the conversion from `TypeNode` to `TypeDirectiveNode` infallible

`TypeDirectiveNode` now implements `From<TypeNode>` instead of `TryFrom<TypeNode>`, since a link becomes a `Link` and every other type node is a registered type node. Calls to `try_from` still compile, with an infallible error.
