---
default: major
---

#### Regenerate the nodes from spec v2

Node structs now follow spec v2: every node carries `plugins`, type nodes carry `transforms` in place of the v1 wrapper nodes, and `NumberTypeNode`/`NumberValueNode` are replaced by `IntegerTypeNode`, `FloatTypeNode`, `FixedPointTypeNode`, `DurationTypeNode`, `IntegerValueNode` and `FloatValueNode`. Named nodes serialise `identifier` instead of `name`, nodes are no longer `Copy`, and the v1-only nodes (`argumentValueNode`, `resolverValueNode`, `instructionArgumentNode`, the three enum variant nodes, `amountTypeNode`, `solAmountTypeNode`) are removed.
