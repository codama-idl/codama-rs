---
default: patch
---

#### Accept the minimum value in signed integer attributes

Attributes reading a signed integer, such as `default_value` or the `offset` of `pre_offset` and `post_offset`, rejected the minimum value of the type (e.g. `-9223372036854775808` for an `i64`) because the sign was applied after parsing the digits. The whole range of the type is now accepted. (#142)
