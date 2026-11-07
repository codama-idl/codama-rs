---
default: major
---

#### Remove `Docs::push`

`Docs` no longer has a `push` method. Build docs from a `Vec<String>` or a `Text` instead, e.g. `Docs::from(vec!["First line".to_string()])`.
