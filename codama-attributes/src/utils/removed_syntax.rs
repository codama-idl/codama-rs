use codama_syn_helpers::extensions::*;
use quote::ToTokens;

/// The error for v1 syntax that is removed in v2.
/// Provides a hint to the v2 alternative replacement.
pub fn removed_syntax<T>(tokens: impl ToTokens, syntax: &str, replacement: &str) -> syn::Result<T> {
    Err(tokens.error(format!("{syntax} is removed in v2, use {replacement}")))
}
