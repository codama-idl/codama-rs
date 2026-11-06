---
default: major
---

#### Emit transforms from wrapper attributes

`fixed_size(...)`, `size_prefix(...)`, `pre_offset(...)`, `post_offset(...)`, `sentinel(...)`, `hidden_prefix(...)` and `hidden_suffix(...)` keep their syntax, but they now attach transforms to the type they wrap, innermost first, rather than producing wrapper nodes in the IDL. They can also wrap a `link(...)`, and the prefix of `size_prefix(...)` must be an `integer(...)`.
