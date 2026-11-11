---
default: minor
---

#### List the expected node kinds in errors

Nodes and node unions now implement `NodeKinds`, whose `KINDS` constant lists the node kinds they can hold, like the `*_KINDS` constants of the JavaScript library. `ProvidedScope` uses it to name the expected kinds when a resolved value has the wrong kind, and reports a fallback of the wrong kind with a new `UnexpectedNodeKind` error.
