# Changelog

Every published crate of the workspace shares this changelog and the same version. Earlier releases are listed on the [GitHub releases](https://github.com/codama-idl/codama-rs/releases) page, under their `<crate>-vX.Y.Z` tags.

## 0.13.3 (2026-11-04)

### Fixes

#### Accept the minimum value in signed integer attributes

Attributes reading a signed integer, such as `default_value` or the `offset` of `pre_offset` and `post_offset`, rejected the minimum value of the type (e.g. `-9223372036854775808` for an `i64`) because the sign was applied after parsing the digits. The whole range of the type is now accepted. (#142)

#### Identify `solana_program::pubkey::Pubkey` as a public key type

A field typed `solana_program::pubkey::Pubkey` was exported as a dangling `definedTypeLinkNode` named `pubkey` instead of a `publicKeyTypeNode`, like the `solana_sdk` and `solana_pubkey` paths already were. (#140)

#### Name the right union in node union deserialize errors

Failing to deserialize any node union reported `failed to deserialize AmountTypeNode`, whichever union was being deserialized. The message now names the actual union, e.g. `failed to deserialize TypeNode`. (#141)

## 0.13.2 (2026-08-24)

### Features

- Add the `remaining_accounts` directive for instructions (#137)
