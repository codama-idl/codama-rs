use crate::utils::FromMeta;
use codama_nodes::{CountNode, FixedCountNode, PrefixedCountNode, RemainderCountNode};
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for CountNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        // A bare integer is the common case, so `3` is short for `fixed_count(3)`.
        if let Ok(expr) = meta.as_expr() {
            if let Ok(value) = expr.as_unsigned_integer() {
                return Ok(FixedCountNode::new(value).into());
            }
        }

        match meta.path_str().as_str() {
            "fixed_count" => FixedCountNode::from_meta(meta).map(Self::from),
            "prefixed_count" => PrefixedCountNode::from_meta(meta).map(Self::from),
            "remainder_count" => RemainderCountNode::from_meta(meta).map(Self::from),
            _ => Err(meta.error("unrecognized count")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use codama_nodes::{NumberFormat::U32, NumberTypeNode};

    fn parse(meta: Meta) -> syn::Result<CountNode> {
        CountNode::from_meta(&meta)
    }

    #[test]
    fn fixed() {
        let expected: CountNode = FixedCountNode::new(3).into();
        assert_eq!(
            parse(syn::parse_quote! { fixed_count(3) }).unwrap(),
            expected
        );
        assert_eq!(
            parse(syn::parse_quote! { fixed_count(value = 3) }).unwrap(),
            expected
        );
    }

    #[test]
    fn bare_integer_is_a_fixed_count() {
        assert_eq!(
            parse(syn::parse_quote! { 3 }).unwrap(),
            FixedCountNode::new(3).into()
        );
    }

    #[test]
    fn prefixed() {
        let expected: CountNode = PrefixedCountNode::new(NumberTypeNode::le(U32)).into();
        assert_eq!(
            parse(syn::parse_quote! { prefixed_count(number(u32)) }).unwrap(),
            expected
        );
        assert_eq!(
            parse(syn::parse_quote! { prefixed_count(prefix = number(u32)) }).unwrap(),
            expected
        );
    }

    #[test]
    fn remainder() {
        let expected: CountNode = RemainderCountNode::new().into();
        assert_eq!(
            parse(syn::parse_quote! { remainder_count }).unwrap(),
            expected
        );
        assert_eq!(
            parse(syn::parse_quote! { remainder_count() }).unwrap(),
            expected
        );
    }

    #[test]
    fn invalid_prefix() {
        let error = parse(syn::parse_quote! { prefixed_count(string) }).unwrap_err();
        assert_eq!(error.to_string(), "prefix must be a NumberTypeNode");
    }

    #[test]
    fn unrecognized_count() {
        let error = parse(syn::parse_quote! { unrecognized }).unwrap_err();
        assert_eq!(error.to_string(), "unrecognized count");
    }
}
