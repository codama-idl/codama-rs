---
default: major
---

#### Fail when modules or plugins describe the same item differently

When several modules describe the same program, two nodes of the same kind and identifier with different content, or two plugins with the same namespace and different payloads, are now an error instead of one silently replacing the other. Identical duplicates still merge into one. A linked PDA seed that matches no field is also an error.
