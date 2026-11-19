---
default: major
---

#### Validate the default values of instruction inputs

`Codama::get_idl` now also fails when the default values of an instruction depend on each other in a cycle or on an account or data field that the instruction does not have, when a data field defaults to the bump of an account that may be a signer, and when an injected value is neither provided nor given a fallback. `codama-visitors-core` gains `get_resolved_instruction_inputs` and `get_instruction_data_fields`, like in the JavaScript library.
