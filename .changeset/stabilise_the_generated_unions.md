---
default: major
---

#### Mark every union and enum as non-exhaustive and name inline union variants after their node

Every generated union and enumeration, such as `ValueNode`, `TransformNode`, `IntegerFormat` and `IsSigner`, as well as `TypeNode` and `RegisteredTypeNode`, is now `#[non_exhaustive]` like `Node`, so a member that a minor spec version adds is not a breaking change. Matches outside of `codama-nodes` need a wildcard arm. The variants of the inline unions no longer depend on their other members: they keep the whole name of their node like `Node` does, so `PdaSeedValueValue::Integer` is now `PdaSeedValueValue::IntegerValue`, and `ContextualValueNode::Account` is `ContextualValueNode::AccountValue`. The main union of each category still implies its category word, e.g. `ValueNode::Integer` and `LinkNode::Account`.
