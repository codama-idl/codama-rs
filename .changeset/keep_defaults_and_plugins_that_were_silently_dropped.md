---
default: patch
---

#### Keep defaults and plugins that were silently dropped

An instruction field renamed with `type = field("...", ...)` now keeps its `default_value`, and the plugins set on a program by several modules are now merged by namespace rather than only keeping those of the first module.
