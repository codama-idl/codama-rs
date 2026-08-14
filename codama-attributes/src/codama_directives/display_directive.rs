use crate::{
    utils::{FromMeta, SetOnce},
    Attribute, CodamaAttribute, CodamaDirective,
};
use codama_errors::CodamaError;
use codama_nodes::{DisplaySkip, NumberDisplayNode};
use codama_syn_helpers::{extensions::*, Meta};

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DisplayDirective {
    pub intent: Option<String>,
    pub interpolated_intent: Option<String>,
    pub label: Option<String>,
    pub skip: Option<DisplaySkip>,
    pub flatten: Option<bool>,
    pub flatten_prefix: Option<String>,
    pub number_display: Option<NumberDisplayNode>,
}

impl DisplayDirective {
    pub fn parse(meta: &Meta) -> syn::Result<Self> {
        let pl = meta.assert_directive("display")?.as_path_list()?;
        let mut intent: SetOnce<String> = SetOnce::new("intent");
        let mut interpolated_intent: SetOnce<String> = SetOnce::new("interpolated_intent");
        let mut label: SetOnce<String> = SetOnce::new("label");
        let mut skip: SetOnce<DisplaySkip> = SetOnce::new("skip");
        let mut flatten: SetOnce<bool> = SetOnce::new("flatten");
        let mut flatten_prefix: SetOnce<String> = SetOnce::new("flatten_prefix");
        let mut number_display: SetOnce<NumberDisplayNode> = SetOnce::new("number display");

        pl.each(|ref meta| match meta.path_str().as_str() {
            "intent" => intent.set(meta.as_value()?.as_expr()?.as_string()?, meta),
            "interpolated_intent" => {
                interpolated_intent.set(meta.as_value()?.as_expr()?.as_string()?, meta)
            }
            "label" => label.set(meta.as_value()?.as_expr()?.as_string()?, meta),
            "skip" => skip.set(DisplaySkip::from_meta(meta.as_value()?)?, meta),
            "flatten" => flatten.set(bool::from_meta(meta)?, meta),
            "flatten_prefix" => flatten_prefix.set(meta.as_value()?.as_expr()?.as_string()?, meta),
            "amount" | "date_time" | "duration" | "injected" => {
                number_display.set(NumberDisplayNode::from_meta(meta)?, meta)
            }
            "string" => Err(meta.error("string display is not supported yet")),
            "skip_inner_data" => Err(meta.error("enum variant display is not supported yet")),
            _ => Err(meta.error("unrecognized display attribute")),
        })?;

        let display = Self {
            intent: intent.option(),
            interpolated_intent: interpolated_intent.option(),
            label: label.option(),
            skip: skip.option(),
            flatten: flatten.option(),
            flatten_prefix: flatten_prefix.option(),
            number_display: number_display.option(),
        };
        if display == Self::default() {
            return Err(meta.error("display requires at least one attribute"));
        }
        Ok(display)
    }
}

impl<'a> TryFrom<&'a CodamaAttribute<'a>> for &'a DisplayDirective {
    type Error = CodamaError;

    fn try_from(attribute: &'a CodamaAttribute) -> Result<Self, Self::Error> {
        match attribute.directive.as_ref() {
            CodamaDirective::Display(ref display) => Ok(display),
            _ => Err(CodamaError::InvalidCodamaDirective {
                expected: "display".to_string(),
                actual: attribute.directive.name().to_string(),
            }),
        }
    }
}

impl<'a> TryFrom<&'a Attribute<'a>> for &'a DisplayDirective {
    type Error = CodamaError;

    fn try_from(attribute: &'a Attribute) -> Result<Self, Self::Error> {
        <&CodamaAttribute>::try_from(attribute)?.try_into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use codama_nodes::{
        AmountNumberDisplayNode, DateTimeNumberDisplayNode, InstructionAccountDisplayNode,
        NumberValueNode, StringValueNode,
    };

    fn parse_display(tokens: proc_macro2::TokenStream) -> syn::Result<DisplayDirective> {
        let meta: Meta = syn::parse2(tokens)?;
        DisplayDirective::parse(&meta)
    }

    #[test]
    fn parses_instruction_display() {
        let display = parse_display(quote::quote! {
            display(
                intent = "Transfer tokens",
                interpolated_intent = "Transfer ${data.amount} to ${accounts.destination}"
            )
        })
        .unwrap();
        assert_eq!(
            display,
            DisplayDirective {
                intent: Some("Transfer tokens".to_string()),
                interpolated_intent: Some(
                    "Transfer ${data.amount} to ${accounts.destination}".to_string()
                ),
                ..Default::default()
            }
        );
    }

    #[test]
    fn parses_struct_field_display() {
        let display = parse_display(quote::quote! {
            display(
                label = "Authorities",
                skip = when_injected,
                flatten,
                flatten_prefix = "Authority "
            )
        })
        .unwrap();
        assert_eq!(
            display,
            DisplayDirective {
                label: Some("Authorities".to_string()),
                skip: Some(DisplaySkip::WhenInjected),
                flatten: Some(true),
                flatten_prefix: Some("Authority ".to_string()),
                ..Default::default()
            }
        );
    }

    #[test]
    fn parses_amount_display() {
        let display = parse_display(quote::quote! {
            display(amount(decimals = 9, unit = "SOL"))
        })
        .unwrap();
        assert_eq!(
            display.number_display,
            Some(NumberDisplayNode::Amount(AmountNumberDisplayNode {
                decimals: Box::new(Some(NumberValueNode::new(9u64).into())),
                unit: Box::new(Some(StringValueNode::new("SOL").into())),
            }))
        );
    }

    #[test]
    fn parses_empty_amount_display() {
        let display = parse_display(quote::quote! { display(amount) }).unwrap();
        assert_eq!(
            display.number_display,
            Some(NumberDisplayNode::Amount(Default::default()))
        );
    }

    #[test]
    fn parses_partial_amount_displays() {
        let decimals = parse_display(quote::quote! {
            display(amount(decimals = 6))
        })
        .unwrap();
        assert_eq!(
            decimals.number_display,
            Some(NumberDisplayNode::Amount(AmountNumberDisplayNode {
                decimals: Box::new(Some(NumberValueNode::new(6u64).into())),
                unit: Box::new(None),
            }))
        );

        let unit = parse_display(quote::quote! {
            display(amount(unit = "bytes"))
        })
        .unwrap();
        assert_eq!(
            unit.number_display,
            Some(NumberDisplayNode::Amount(AmountNumberDisplayNode {
                decimals: Box::new(None),
                unit: Box::new(Some(StringValueNode::new("bytes").into())),
            }))
        );
    }

    #[test]
    fn parses_date_time_display() {
        let display = parse_display(quote::quote! {
            display(date_time(ticks_per_second = 1_000))
        })
        .unwrap();
        assert_eq!(
            display.number_display,
            Some(NumberDisplayNode::DateTime(DateTimeNumberDisplayNode {
                ticks_per_second: Some(1_000),
            }))
        );
    }

    #[test]
    fn parses_empty_date_time_display() {
        let display = parse_display(quote::quote! { display(date_time) }).unwrap();
        assert_eq!(
            display.number_display,
            Some(NumberDisplayNode::DateTime(Default::default()))
        );
    }

    #[test]
    fn parses_instruction_account_display() {
        let meta: Meta = syn::parse_quote! { display(label = "Payer", skip = always) };
        assert_eq!(
            InstructionAccountDisplayNode::from_meta(&meta).unwrap(),
            InstructionAccountDisplayNode {
                label: Some("Payer".to_string()),
                skip: Some(DisplaySkip::Always),
            }
        );
    }

    #[test]
    fn parses_flatten_prefix_without_flatten() {
        let display = parse_display(quote::quote! {
            display(flatten_prefix = "Authority ")
        })
        .unwrap();
        assert_eq!(display.flatten, None);
        assert_eq!(display.flatten_prefix, Some("Authority ".to_string()));
    }

    #[test]
    fn parses_explicit_false_flatten() {
        let display = parse_display(quote::quote! { display(flatten = false) }).unwrap();
        assert_eq!(display.flatten, Some(false));
    }

    #[test]
    fn rejects_multiple_number_displays() {
        let error = parse_display(quote::quote! {
            display(amount, date_time)
        })
        .unwrap_err();
        assert_eq!(error.to_string(), "number display is already set");
    }

    #[test]
    fn rejects_unsupported_display_nodes() {
        for (tokens, expected) in [
            (
                quote::quote! { display(duration) },
                "duration display is not supported yet",
            ),
            (
                quote::quote! { display(string) },
                "string display is not supported yet",
            ),
            (
                quote::quote! { display(skip_inner_data = true) },
                "enum variant display is not supported yet",
            ),
        ] {
            assert_eq!(parse_display(tokens).unwrap_err().to_string(), expected);
        }
    }

    #[test]
    fn rejects_injected_amount_values() {
        let error = parse_display(quote::quote! {
            display(amount(decimals = injected("mint_decimals")))
        })
        .unwrap_err();
        assert_eq!(
            error.to_string(),
            "injected display values are not supported yet"
        );
    }

    #[test]
    fn rejects_positional_attributes() {
        for tokens in [
            quote::quote! { display("Transfer tokens") },
            quote::quote! { display(amount(9, "SOL")) },
            quote::quote! { display(date_time(1_000)) },
        ] {
            assert!(parse_display(tokens).is_err());
        }
    }

    #[test]
    fn rejects_invalid_skip() {
        let error = parse_display(quote::quote! { display(skip = sometimes) }).unwrap_err();
        assert_eq!(
            error.to_string(),
            "expected always, never, or when_injected"
        );
    }

    #[test]
    fn rejects_empty_display() {
        let error = parse_display(quote::quote! { display() }).unwrap_err();
        assert_eq!(error.to_string(), "display requires at least one attribute");
    }
}
