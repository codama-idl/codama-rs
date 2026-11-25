---
default: major
---

#### Name the file in filesystem and manifest errors

`CodamaError::Filesystem` and `CodamaError::Manifest` now carry the `path` of the file that could not be read or parsed alongside their `source`, so `Could not read `programs/token/src/lib.rs`: No such file or directory` replaces a bare OS error. They are no longer built with `From`, and `CodamaError::UnexpectedNode` is merged into `UnexpectedNodeKind`. `CodamaError::to_compile_error` now emits a `compile_error!` for every variant rather than nothing for those without a span.
