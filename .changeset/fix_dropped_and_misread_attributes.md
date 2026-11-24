---
default: major
---

#### Stop dropping or misreading attributes

- The `docs` of a `field(...)` type reach the IDL instead of being parsed and dropped.
- `skip` rejects an argument, so `#[codama(skip = false)]` is an error rather than a skip, and `option(..., fixed(false))` is an error rather than a fixed option.
- Attributes behind any `cfg_attr` condition, such as `#[cfg_attr(not(target_os = "solana"), codama(...))]`, are read like those behind a feature, as the IDL describes every configuration.
- An invalid PDA seed reports what a seed can be instead of an internal message.
