use codama_syn_helpers::extensions::*;
use quote::ToTokens;

/// The error for v1 syntax that is removed in v2.
/// Provides a hint to the v2 alternative replacement.
pub fn removed_syntax<T>(tokens: impl ToTokens, syntax: &str, replacement: &str) -> syn::Result<T> {
    Err(tokens.error(format!("{syntax} is removed in v2, use {replacement}")))
}

/// The error for the v1 `name = ...` key. v2 uses `identifier = ...`, the name of the spec attribute.
pub fn removed_name_key<T>(tokens: impl ToTokens) -> syn::Result<T> {
    removed_syntax(tokens, "name = ...", "identifier = ...")
}
