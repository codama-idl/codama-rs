---
default: major
---

#### Point attribute errors at their cause

Error messages now name the actual problem instead of a generic one: an out-of-range number in `default_value = ...` reports the range rather than `unrecognized value`, `scale` and `ticks_per_second` name their limit, a Rust name with no valid identifier explains why and suggests `identifier = "..."`, an invalid identifier states the identifier rule, and integer slots ask for an `integer(...)` type rather than an `IntegerTypeNode`. `data(name = "...")` points to `data("...")`, and conflicting plugins between modules name the module. Code matching on these messages needs updating.
