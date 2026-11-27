---
default: patch
---

#### Report unrecognized attributes consistently

An unknown key in `account(...)` or `account_bump(...)`, such as `account(banana = "authority")`, now fails with `unrecognized attribute` like every other directive, rather than `only 'identifier' attribute supported`. Within `integer(...)` and `float(...)`, the error now underlines the whole unrecognized attribute, such as `foo = 42` rather than `foo` alone, and a value that is not a path, such as `integer(u64, "slots")`, also fails with `unrecognized attribute` rather than `expected a path`.
