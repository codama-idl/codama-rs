---
default: major
---

#### Make `CodamaError` non-exhaustive

New error variants can now be added in minor releases, so a `match` on `CodamaError` outside this crate needs a wildcard arm.
