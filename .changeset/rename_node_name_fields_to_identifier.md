---
default: major
---

#### Rename the `name` field of nodes to `identifier`

Following spec v2, the 22 nodes that identify themselves by a name now expose that name as `identifier` rather than `name`, and the `HasName` trait becomes `HasIdentifier` with an `identifier()` method. This only affects code constructing or reading nodes directly, e.g. plugins: the `#[codama(...)]` attributes and the generated IDL are unchanged by this step.
