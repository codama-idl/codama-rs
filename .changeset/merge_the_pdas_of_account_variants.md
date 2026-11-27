---
default: patch
---

#### Report conflicting PDAs of account variants against their enum

Variants of a `#[derive(CodamaAccounts)]` enum sharing an identical PDA now keep it once rather than adding it to the program for each variant, and variants describing a PDA of the same identifier differently fail with an error pointing at the enum that declares them.
