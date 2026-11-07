---
default: major
---

#### Build identifiers, paths and namespaces from string literals

`Identifier`, `Path`, `Namespace` and `Decimal` convert from string literals, so constructors take them directly, e.g. `AccountNode::new("my_account", data)` or `FieldDiscriminatorNode::new("header.kind", 0)`. A literal that breaks the spec grammar panics; parse any other string with `str::parse` or `TryFrom<String>`, which return the error instead.
