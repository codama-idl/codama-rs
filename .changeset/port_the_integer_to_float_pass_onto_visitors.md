---
default: minor
---

#### Run the integer-to-float pass on the visitors of `codama-visitors-core`

`TransformIntegersToFloatsVisitor` now rebuilds the root node with an `IdentityVisitor`, resolving types through the `LinkableDictionary`, and still runs in the default plugin. Its logic is available as `transform_integers_to_floats(&root)`, so a plugin that adds values later can run it again. It now also reaches every constant value and every PDA value of the tree, such as a constant used as a field default or a PDA value within a conditional default. `codama-visitors-core` gains `resolve_defined_type_links`.
