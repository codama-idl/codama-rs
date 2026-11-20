---
default: patch
---

#### Report conflicting PDAs of account variants against their enum

Variants of a `#[derive(CodamaAccounts)]` enum sharing an identical PDA keep it once, and variants describing a PDA of the same identifier differently now fail with the enum that declares them instead of the program.
