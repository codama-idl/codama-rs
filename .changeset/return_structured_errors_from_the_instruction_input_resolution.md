---
default: major
---

#### Return a structured error from `get_resolved_instruction_inputs`

`get_resolved_instruction_inputs` now fails with a `ResolveInstructionInputsError` that keeps the node paths and the `InstructionDependency` it refers to, like `resolve_type_path` does with `ResolveTypePathError`, so callers match on them rather than on the strings of a `CodamaError`. It still converts into a `CodamaError`, so `?` keeps working. `LinkableDictionary::get_path_of::<T>` returns the linked node as the type its link records, `NodePath::until_last_with::<T>` returns the node along with the path, and `NodePath::child` copies a path with a node added.
