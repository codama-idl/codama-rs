---
default: major
---

#### Simplify enum value constructors and directive checks

`EnumValueNode` now has two constructors: `new` builds a variant without data, and `with_value` builds a variant with any value as its data. The previous `empty`, `fields` and `tuple` constructors are removed. `Attributes::has_codama_attribute` is replaced by `Attributes::has`, which takes a directive filter such as `SkipDirective::filter` instead of a directive name. Invalid identifiers in attributes now point at the string itself.
