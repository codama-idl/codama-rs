---
default: patch
---

#### Keep the stack frames of `map_children` small

`NodeRef::map_children` now rebuilds each node kind in its own out-of-line function instead of a single match over every kind, whose stack frame took about 260 KB per level of the tree in debug builds. An `IdentityVisitor` pass therefore no longer overflows the 2 MB stack of a thread on a tree only 8 levels deep, and uses about 10 KB per level in debug builds and 3 KB in release builds.
