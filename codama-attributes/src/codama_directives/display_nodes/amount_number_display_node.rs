use crate::utils::{FromMeta, SetOnce};
use codama_nodes::{
    AmountNumberDisplayNode, InjectableNumberValueNode, InjectableStringValueNode, NumberValueNode,
    StringValueNode,
};
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for AmountNumberDisplayNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        meta.assert_directive("amount")?;
        if meta.is_path_or_empty_list() {
            return Ok(Self::default());
        }
        let pl = meta.as_path_list()?;
        let mut decimals: SetOnce<InjectableNumberValueNode> = SetOnce::new("decimals");
        let mut unit: SetOnce<InjectableStringValueNode> = SetOnce::new("unit");

        pl.each(|ref meta| match meta.path_str().as_str() {
            "decimals" => {
                let value = meta.as_value()?;
                reject_injected_value(value)?;
                decimals.set(
                    NumberValueNode::new(value.as_expr()?.as_unsigned_integer::<u64>()?).into(),
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
            decimals: Box::new(decimals.option()),
            unit: Box::new(unit.option()),
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
            AmountNumberDisplayNode {
                decimals: Box::new(Some(NumberValueNode::new(9u64).into())),
                unit: Box::new(Some(StringValueNode::new("SOL").into())),
            }
        );
    }

    #[test]
    fn parses_empty_display() {
        for meta in [syn::parse_quote! { amount }, syn::parse_quote! { amount() }] {
            assert_eq!(
                AmountNumberDisplayNode::from_meta(&meta).unwrap(),
                AmountNumberDisplayNode::default()
            );
        }
    }

    #[test]
    fn parses_partial_displays() {
        let meta: Meta = syn::parse_quote! { amount(decimals = 6) };
        assert_eq!(
            AmountNumberDisplayNode::from_meta(&meta).unwrap(),
            AmountNumberDisplayNode {
                decimals: Box::new(Some(NumberValueNode::new(6u64).into())),
                unit: Box::new(None),
            }
        );

        let meta: Meta = syn::parse_quote! { amount(unit = "bytes") };
        assert_eq!(
            AmountNumberDisplayNode::from_meta(&meta).unwrap(),
            AmountNumberDisplayNode {
                decimals: Box::new(None),
                unit: Box::new(Some(StringValueNode::new("bytes").into())),
            }
        );
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
