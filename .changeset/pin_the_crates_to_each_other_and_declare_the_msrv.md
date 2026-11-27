---
default: patch
---

#### Pin the crates to each other and declare the minimum Rust version

The crates of the workspace now depend on each other at their exact version, so a release candidate never resolves to a sibling from another candidate, and they declare Rust 1.83 as their minimum supported version. `codama-rs generate-idl -o <file>` ends the file with a newline, like its output on stdout.
