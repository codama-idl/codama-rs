---
default: major
---

#### Update count, discriminator and display node constructors for spec v2

`FieldDiscriminatorNode` now points at its field with a `Path`, validated against the spec path grammar like the `path` of `dataValueNode` and `accountDataValueNode`, so a nested field such as `header.kind` can serve as a discriminator. `AmountNumberDisplayNode::new` requires `decimals` and no longer implements `Default`, and `PrefixedCountNode::new` takes an `IntegerTypeNode`. New constructors are available for `SentinelCountNode`, `UnitNumberDisplayNode` and `ValueLabelDisplayNode`, along with `IntegerTypeNode::{le, be}` and `IntegerValueNode::new`.
