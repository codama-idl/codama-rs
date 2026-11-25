---
default: major
---

#### Name the file in filesystem and manifest errors

`CodamaError::Filesystem` and `CodamaError::Manifest` now carry the `path` of the file that could not be accessed or parsed alongside their `source`, which is also the cause of the error. A bare OS error is replaced by one naming the file:

```text
Could not access `programs/token/src/lib.rs`: No such file or directory (os error 2)
```

They are no longer built with `From`, and `CodamaError::UnexpectedNode` is merged into `UnexpectedNodeKind`. `CodamaError::to_compile_error` now emits a `compile_error!` for every variant rather than nothing for those without a span.
