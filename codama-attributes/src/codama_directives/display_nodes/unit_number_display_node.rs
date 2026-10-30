use crate::utils::{FromMeta, SetOnce};
use codama_nodes::{InjectableUnitValueNode, UnitNumberDisplayNode};
use codama_syn_helpers::Meta;

impl FromMeta for UnitNumberDisplayNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        let pl = meta.assert_directive("unit")?.as_path_list()?;
        let mut unit = SetOnce::<InjectableUnitValueNode>::new("unit");
        pl.each(|ref meta| unit.set(InjectableUnitValueNode::from_meta(meta)?, meta))?;
        Ok(Self::new(unit.take(meta)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use codama_nodes::{PublicKeyValueNode, StringValueNode};

    #[test]
    fn parses_display() {
        let meta: Meta = syn::parse_quote! { unit("SOL") };
        assert_eq!(
            UnitNumberDisplayNode::from_meta(&meta).unwrap(),
            UnitNumberDisplayNode::new(StringValueNode::new("SOL"))
        );
    }

    #[test]
    fn parses_address_units() {
        for (meta, expected) in [
            (
                syn::parse_quote! { unit(public_key("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v")) },
                PublicKeyValueNode::new("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"),
            ),
            (
                syn::parse_quote! { unit(program("system")) },
                PublicKeyValueNode::new("11111111111111111111111111111111"),
            ),
            (
                syn::parse_quote! { unit(sysvar("clock")) },
                PublicKeyValueNode::new("SysvarC1ock11111111111111111111111111111111"),
            ),
        ] {
            assert_eq!(
                UnitNumberDisplayNode::from_meta(&meta).unwrap(),
                UnitNumberDisplayNode::new(expected)
            );
        }
    }

    #[test]
    fn errors() {
        for (meta, expected) in [
            (
                syn::parse_quote! { unit },
                "expected a list: `unit(...)` or `unit = (...)`",
            ),
            (syn::parse_quote! { unit() }, "unit is missing"),
            (
                syn::parse_quote! { unit(SOL) },
                "expected a string or a public key",
            ),
            (
                syn::parse_quote! { unit(42) },
                "expected a string or a public key",
            ),
            (
                syn::parse_quote! { unit(true) },
                "expected a string or a public key",
            ),
            (
                syn::parse_quote! { unit(unit = "SOL") },
                "expected a string or a public key",
            ),
            (
                syn::parse_quote! { unit("SOL", "USDC") },
                "unit is already set",
            ),
            (
                syn::parse_quote! { unit("SOL", public_key("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v")) },
                "unit is already set",
            ),
            (
                syn::parse_quote! { unit(injected("mint")) },
                "injected display values are not supported yet",
            ),
            (
                syn::parse_quote! { unit(program("invalid_program")) },
                "unrecognized program",
            ),
        ] {
            let error = UnitNumberDisplayNode::from_meta(&meta).unwrap_err();
            assert_eq!(error.to_string(), expected);
        }
    }
}
