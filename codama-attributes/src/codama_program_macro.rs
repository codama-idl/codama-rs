use crate::utils::{removed_name_key, SetOnce};
use codama_nodes::Identifier;
use codama_syn_helpers::{extensions::*, Meta};
use proc_macro2::TokenStream;

/// The parsed arguments of a `codama_program!` function-like macro invocation.
///
/// Unlike the item-level `#[codama(program(...))]` directive — which declares a
/// *distinct* program and therefore requires both `identifier` and `address` — the
/// `codama_program!` macro overrides the *primary* program's metadata. Both
/// fields are optional (though at least one is required); any field left unset
/// keeps the crate-derived default (the Cargo.toml package name and the
/// `declare_id!` / `package.metadata.solana.program-id` address).
#[derive(Debug, PartialEq)]
pub struct CodamaProgramMacro {
    pub name: Option<Identifier>,
    pub address: Option<String>,
}

impl CodamaProgramMacro {
    /// Parse the raw token stream passed to `codama_program!(...)`.
    pub fn parse(tokens: TokenStream) -> syn::Result<Self> {
        // Reuse the `Meta` grammar by wrapping the arguments in a path list.
        let meta: Meta = syn::parse_quote! { codama_program(#tokens) };
        let pl = meta.as_path_list()?;

        let mut name = SetOnce::<Identifier>::new("identifier");
        let mut address = SetOnce::<String>::new("address");

        pl.each(|ref meta| match meta.path_str().as_str() {
            "identifier" => name.set(
                Identifier::try_from(meta.as_value()?.as_expr()?.as_string()?)
                    .map_err(|e| meta.error(e))?,
                meta,
            ),
            "name" => removed_name_key(meta),
            "address" => address.set(meta.as_value()?.as_expr()?.as_string()?, meta),
            _ => Err(meta.error("unrecognized attribute")),
        })?;

        let name = name.option();
        let address = address.option();
        if name.is_none() && address.is_none() {
            return Err(meta.error("expected at least one of `identifier` or `address`"));
        }

        Ok(Self { name, address })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use codama_nodes::test_utils::id;
    use quote::quote;

    #[test]
    fn identifier_only() {
        let program = CodamaProgramMacro::parse(quote! { identifier = "associatedToken" }).unwrap();
        assert_eq!(
            program,
            CodamaProgramMacro {
                name: Some(id("associatedToken")),
                address: None,
            }
        );
    }

    #[test]
    fn address_only() {
        let program = CodamaProgramMacro::parse(
            quote! { address = "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL" },
        )
        .unwrap();
        assert_eq!(
            program,
            CodamaProgramMacro {
                name: None,
                address: Some("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL".to_string()),
            }
        );
    }

    #[test]
    fn identifier_and_address() {
        let program = CodamaProgramMacro::parse(
            quote! { identifier = "associatedToken", address = "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL" },
        )
        .unwrap();
        assert_eq!(
            program,
            CodamaProgramMacro {
                name: Some(id("associatedToken")),
                address: Some("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL".to_string()),
            }
        );
    }

    #[test]
    fn empty() {
        let error = CodamaProgramMacro::parse(quote! {}).unwrap_err();
        assert_eq!(
            error.to_string(),
            "expected at least one of `identifier` or `address`"
        );
    }

    #[test]
    fn unrecognized_attribute() {
        let error = CodamaProgramMacro::parse(quote! { identifier = "foo", version = "1.0.0" })
            .unwrap_err();
        assert_eq!(error.to_string(), "unrecognized attribute");
    }

    #[test]
    fn name_is_removed() {
        let error = CodamaProgramMacro::parse(quote! { name = "token" }).unwrap_err();
        assert_eq!(
            error.to_string(),
            "name = ... is removed in v2, use identifier = ..."
        );
    }
}
