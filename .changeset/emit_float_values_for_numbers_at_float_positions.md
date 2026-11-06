---
default: minor
---

#### Emit float values for numbers written at float positions

A whole number such as `default_value = 1` on a float field, constant, discriminator or PDA seed now produces a `floatValueNode` rather than an `integerValueNode`, following links to find the type, so the output matches spec v2 and `@codama/upgrade`.
