use crate::utils::FromMeta;
use codama_nodes::{InjectableUnitValueNode, ValueNode};
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for InjectableUnitValueNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        let path = meta.path_str();
        if path == "injected" {
            return Err(meta.error("injected display values are not supported yet"));
        }
        match ValueNode::from_meta(meta) {
            Ok(ValueNode::String(node)) => Ok(node.into()),
            Ok(ValueNode::PublicKey(node)) => Ok(node.into()),
            // Public key keywords keep their own errors, such as an unrecognized program.
            Err(error) if matches!(path.as_str(), "public_key" | "program" | "sysvar") => {
                Err(error)
            }
            _ => Err(meta.error("expected a string or a public key")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use codama_nodes::{PublicKeyValueNode, StringValueNode};

    #[test]
    fn parses_units() {
        for (meta, expected) in [
            (
                syn::parse_quote! { "SOL" },
                InjectableUnitValueNode::from(StringValueNode::new("SOL")),
            ),
            (
                syn::parse_quote! { public_key("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v") },
                PublicKeyValueNode::new("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v").into(),
            ),
            (
                syn::parse_quote! { program("token") },
                PublicKeyValueNode::new("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").into(),
            ),
            (
                syn::parse_quote! { sysvar("rent") },
                PublicKeyValueNode::new("SysvarRent111111111111111111111111111111111").into(),
            ),
        ] {
            assert_eq!(InjectableUnitValueNode::from_meta(&meta).unwrap(), expected);
        }
    }

    #[test]
    fn errors() {
        for (meta, expected) in [
            (
                syn::parse_quote! { 42 },
                "expected a string or a public key",
            ),
            (
                syn::parse_quote! { 1.5 },
                "expected a string or a public key",
            ),
            (
                syn::parse_quote! { true },
                "expected a string or a public key",
            ),
            (
                syn::parse_quote! { SOL },
                "expected a string or a public key",
            ),
            (
                syn::parse_quote! { unit = "SOL" },
                "expected a string or a public key",
            ),
            (
                syn::parse_quote! { injected("mint") },
                "injected display values are not supported yet",
            ),
            (
                syn::parse_quote! { program("invalid_program") },
                "unrecognized program",
            ),
            (
                syn::parse_quote! { sysvar("invalid_sysvar") },
                "unrecognized sysvar",
            ),
            (syn::parse_quote! { public_key(SOL) }, "expected a string"),
        ] {
            let error = InjectableUnitValueNode::from_meta(&meta).unwrap_err();
            assert_eq!(error.to_string(), expected);
        }
    }
}
