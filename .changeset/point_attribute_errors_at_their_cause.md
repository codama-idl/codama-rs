---
default: major
---

#### Point attribute errors at their cause

A number in `default_value = ...` too large or too small for any integer now says so rather than `unrecognized value`, and a Rust name that yields no valid identifier, such as `été`, explains why and suggests setting one with `identifier = "..."`. Code matching on these messages needs updating.
