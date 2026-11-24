---
default: patch
---

#### Depend on data field program ids and report optional seeds from their account

A PDA value whose `programId` is a data field now depends on that field, so the field resolves before the PDA value instead of possibly after it. The error for an optional account used as a PDA seed starts its path at the instruction account whenever the PDA value is the account's own, including when resolving an injected seed within it rebuilt the value, rather than only when nothing was injected.
