use crate::utils::{FromMeta, SetOnce};
use codama_nodes::{
    AmountNumberDisplayNode, InjectableIntegerValueNode, InjectableUnitValueNode, IntegerValueNode,
    StringValueNode,
};
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for AmountNumberDisplayNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        meta.assert_directive("amount")?;
        if meta.is_path_or_empty_list() {
            return Err(meta.error("decimals is missing"));
        }
        let pl = meta.as_path_list()?;
        let mut decimals: SetOnce<InjectableIntegerValueNode> = SetOnce::new("decimals");
        let mut unit: SetOnce<InjectableUnitValueNode> = SetOnce::new("unit");

        pl.each(|ref meta| match meta.path_str().as_str() {
            "decimals" => {
                let value = meta.as_value()?;
                reject_injected_value(value)?;
                decimals.set(
                    IntegerValueNode::new(value.as_expr()?.as_unsigned_integer::<u64>()?).into(),
                    meta,
                )
            }
            "unit" => {
                let value = meta.as_value()?;
                reject_injected_value(value)?;
                unit.set(
                    StringValueNode::new(value.as_expr()?.as_string()?).into(),
                    meta,
                )
            }
            "injected" => Err(meta.error("injected display values are not supported yet")),
            _ => Err(meta.error("unrecognized amount display attribute")),
        })?;

        Ok(Self {
            decimals: Box::new(decimals.take(meta)?),
            unit: Box::new(unit.option()),
            plugins: vec![],
        })
    }
}

fn reject_injected_value(meta: &Meta) -> syn::Result<()> {
    if meta.path_str() == "injected" {
        Err(meta.error("injected display values are not supported yet"))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_display() {
        let meta: Meta = syn::parse_quote! { amount(decimals = 9, unit = "SOL") };
        assert_eq!(
            AmountNumberDisplayNode::from_meta(&meta).unwrap(),
            AmountNumberDisplayNode::new(IntegerValueNode::new(9u64), StringValueNode::new("SOL"))
        );
    }

    #[test]
    fn parses_display_without_unit() {
        let meta: Meta = syn::parse_quote! { amount(decimals = 6) };
        assert_eq!(
            AmountNumberDisplayNode::from_meta(&meta).unwrap(),
            AmountNumberDisplayNode {
                decimals: Box::new(IntegerValueNode::new(6u64).into()),
                unit: Box::new(None),
                plugins: vec![],
            }
        );
    }

    #[test]
    fn rejects_display_without_decimals() {
        for meta in [
            syn::parse_quote! { amount },
            syn::parse_quote! { amount() },
            syn::parse_quote! { amount(unit = "bytes") },
        ] {
            assert_eq!(
                AmountNumberDisplayNode::from_meta(&meta)
                    .unwrap_err()
                    .to_string(),
                "decimals is missing"
            );
        }
    }

    #[test]
    fn rejects_injected_values() {
        let meta: Meta = syn::parse_quote! { amount(decimals = injected("mint_decimals")) };
        assert_eq!(
            AmountNumberDisplayNode::from_meta(&meta)
                .unwrap_err()
                .to_string(),
            "injected display values are not supported yet"
        );
    }

    #[test]
    fn rejects_positional_attributes() {
        let meta: Meta = syn::parse_quote! { amount(9, "SOL") };
        assert!(AmountNumberDisplayNode::from_meta(&meta).is_err());
    }
}
