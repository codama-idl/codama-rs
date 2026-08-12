use crate::utils::FromMeta;
use codama_nodes::{AmountNumberDisplayNode, DateTimeNumberDisplayNode, NumberDisplayNode};
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for NumberDisplayNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        match meta.path_str().as_str() {
            "amount" => AmountNumberDisplayNode::from_meta(meta).map(Self::Amount),
            "date_time" => DateTimeNumberDisplayNode::from_meta(meta).map(Self::DateTime),
            "duration" => Err(meta.error("duration display is not supported yet")),
            "injected" => Err(meta.error("injected display values are not supported yet")),
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

        let date_time: Meta = syn::parse_quote! { date_time };
        assert!(matches!(
            NumberDisplayNode::from_meta(&date_time).unwrap(),
            NumberDisplayNode::DateTime(_)
        ));
    }

    #[test]
    fn rejects_unsupported_displays() {
        let duration: Meta = syn::parse_quote! { duration };
        assert_eq!(
            NumberDisplayNode::from_meta(&duration)
                .unwrap_err()
                .to_string(),
            "duration display is not supported yet"
        );

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
