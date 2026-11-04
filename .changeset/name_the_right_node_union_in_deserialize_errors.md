---
default: patch
---

#### Name the right union in node union deserialize errors

Failing to deserialize any node union reported `failed to deserialize AmountTypeNode`, whichever union was being deserialized. The message now names the actual union, e.g. `failed to deserialize TypeNode`. (#141)
