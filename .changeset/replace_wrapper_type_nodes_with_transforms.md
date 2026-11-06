---
default: major
---

#### Replace wrapper type nodes with transforms

Layout modifiers no longer wrap the type they apply to: `FixedSizeTypeNode`, `SizePrefixTypeNode`, `PreOffsetTypeNode`, `PostOffsetTypeNode`, `SentinelTypeNode`, `HiddenPrefixTypeNode` and `HiddenSuffixTypeNode` become the `*TransformNode`s of spec v2, attached to any type node or defined type link with `add_transform`, e.g. `StringTypeNode::utf8().add_transform(FixedSizeTransformNode::new(42))`. Floats move to the new `FloatTypeNode`, `FixedPointTypeNode` and `DurationTypeNode` are added, and `DateTimeTypeNode` now wraps an `IntegerTypeNode`.
