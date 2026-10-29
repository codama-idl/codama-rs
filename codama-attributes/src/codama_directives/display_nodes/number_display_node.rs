use crate::utils::{removed_syntax, FromMeta};
use codama_nodes::{AmountNumberDisplayNode, NumberDisplayNode, UnitNumberDisplayNode};
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for NumberDisplayNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        match meta.path_str().as_str() {
            "amount" => AmountNumberDisplayNode::from_meta(meta).map(Self::Amount),
            "date_time" => removed_syntax(meta, "date_time display", "the date_time(...) type"),
            "duration" => removed_syntax(meta, "duration display", "the duration(...) type"),
            "injected" => Err(meta.error("injected display values are not supported yet")),
            "unit" => UnitNumberDisplayNode::from_meta(meta).map(Self::Unit),
            _ => Err(meta.error("unrecognized number display")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_supported_displays() {
        let amount: Meta = syn::parse_quote! { amount(decimals = 9, unit = "SOL") };
        assert!(matches!(
            NumberDisplayNode::from_meta(&amount).unwrap(),
            NumberDisplayNode::Amount(_)
        ));

        let unit: Meta = syn::parse_quote! { unit("SOL") };
        assert!(matches!(
            NumberDisplayNode::from_meta(&unit).unwrap(),
            NumberDisplayNode::Unit(_)
        ));
    }

    #[test]
    fn removed_displays() {
        for meta in [
            syn::parse_quote! { date_time },
            syn::parse_quote! { date_time(ticks_per_second = 1000) },
        ] {
            let error = NumberDisplayNode::from_meta(&meta).unwrap_err();
            assert_eq!(
                error.to_string(),
                "date_time display is removed in v2, use the date_time(...) type"
            );
        }
        for meta in [
            syn::parse_quote! { duration },
            syn::parse_quote! { duration(ticks_per_second = 1000) },
        ] {
            let error = NumberDisplayNode::from_meta(&meta).unwrap_err();
            assert_eq!(
                error.to_string(),
                "duration display is removed in v2, use the duration(...) type"
            );
        }
    }

    #[test]
    fn rejects_unsupported_displays() {
        let string: Meta = syn::parse_quote! { string };
        assert_eq!(
            NumberDisplayNode::from_meta(&string)
                .unwrap_err()
                .to_string(),
            "unrecognized number display"
        );

        let injected: Meta = syn::parse_quote! { injected("decimals") };
        assert_eq!(
            NumberDisplayNode::from_meta(&injected)
                .unwrap_err()
                .to_string(),
            "injected display values are not supported yet"
        );
    }
}
