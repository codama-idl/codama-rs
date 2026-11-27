---
default: minor
---

#### Emit float values for numbers written at float positions

A whole number such as `default_value = 1` on a float field, constant, discriminator or PDA seed now produces a `floatValueNode` rather than an `integerValueNode`, following links to find the type, so the output matches spec v2 and `@codama/upgrade`. The default plugin runs this pass with `TransformIntegersToFloatsVisitor`, and its logic is available as `transform_integers_to_floats(&root)`, so a plugin that adds values later can run it again.
