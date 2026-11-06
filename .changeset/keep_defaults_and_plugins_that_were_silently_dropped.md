---
default: patch
---

#### Keep defaults and plugins that were silently dropped

An instruction field renamed with `type = field("...", ...)` now keeps its `default_value`, and plugins set on a program by several modules are now merged instead of only keeping those of the first module.
