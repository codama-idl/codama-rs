---
default: major
---

#### Call the identifiers of directives `identifier` and parse encodings as bare paths

The `name` field of `AccountDirective`, `FieldDirective`, `ProgramDirective`, `EnumDiscriminatorDirective`, `CodamaProgramMacro` and `SeedDirectiveType::Variable` is renamed `identifier`, as it holds an `Identifier` and the attributes already say `identifier = "..."`. An encoding is written as a bare path everywhere, so `discriminator(bytes = "...", encoding = utf8)` replaces `encoding = "utf8"`, like `string(encoding = utf8)` and `#[codama(encoding = utf8)]` already were, and an invalid one lists the accepted encodings. The `assert_type!` and sibling test macros are no longer exported.
