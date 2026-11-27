---
default: major
---

#### Validate the PDA, instruction and instruction account links of the IDL

`Codama::get_idl` now also fails when a PDA, instruction or instruction account link points to no node of its program, e.g. `#[codama(pda = "vualt")]`. Links naming a program that is not in the IDL point outside of it, such as the PDA of the associated token program, so they are not checked, and neither are account and defined type links to such programs.
