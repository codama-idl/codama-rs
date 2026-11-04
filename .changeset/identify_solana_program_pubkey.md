---
default: patch
---

#### Identify `solana_program::pubkey::Pubkey` as a public key type

A field typed `solana_program::pubkey::Pubkey` was exported as a dangling `definedTypeLinkNode` named `pubkey` instead of a `publicKeyTypeNode`, like the `solana_sdk` and `solana_pubkey` paths already were. (#140)
