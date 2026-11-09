use crate::utils::{parse_number_type, FromMeta};
use codama_nodes::{IntegerFormat, IntegerTypeNode, NumberDisplayNode};
use codama_syn_helpers::Meta;

impl FromMeta for IntegerTypeNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        let arguments =
            parse_number_type::<IntegerFormat, _>(meta, "integer", NumberDisplayNode::from_meta)?;
        Ok(IntegerTypeNode {
            unit: arguments.unit,
            display: Box::new(arguments.display),
            ..IntegerTypeNode::new(arguments.format, arguments.endian)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use codama_nodes::{
        AmountNumberDisplayNode,
        IntegerFormat::{U16, U64},
        IntegerValueNode, PublicKeyValueNode, StringValueNode, UnitNumberDisplayNode,
    };

    #[test]
    fn parses() {
        for (meta, expected) in [
            (syn::parse_quote! { integer(u16) }, IntegerTypeNode::le(U16)),
            (
                syn::parse_quote! { integer(u16, le) },
                IntegerTypeNode::le(U16),
            ),
            (
                syn::parse_quote! { integer(u64, le) },
                IntegerTypeNode::le(U64),
            ),
            (
                syn::parse_quote! { integer(u16, be) },
                IntegerTypeNode::be(U16),
            ),
            (
                syn::parse_quote! { integer(u64, be) },
                IntegerTypeNode::be(U64),
            ),
            (
                syn::parse_quote! { integer(le, u16) },
                IntegerTypeNode::le(U16),
            ),
            (
                syn::parse_quote! { integer(format = u16) },
                IntegerTypeNode::le(U16),
            ),
            (
                syn::parse_quote! { integer(format = u16, endian = le) },
                IntegerTypeNode::le(U16),
            ),
            (
                syn::parse_quote! { integer(format = u64, endian = le) },
                IntegerTypeNode::le(U64),
            ),
            (
                syn::parse_quote! { integer(format = u16, endian = be) },
                IntegerTypeNode::be(U16),
            ),
            (
                syn::parse_quote! { integer(format = u64, endian = be) },
                IntegerTypeNode::be(U64),
            ),
            (
                syn::parse_quote! { integer(endian = le, format = u16) },
                IntegerTypeNode::le(U16),
            ),
            (
                syn::parse_quote! { integer(u64, unit = "slots") },
                IntegerTypeNode {
                    unit: Some("slots".to_string()),
                    ..IntegerTypeNode::le(U64)
                },
            ),
            (
                syn::parse_quote! { integer(unit = "bps", format = u16, endian = be) },
                IntegerTypeNode {
                    unit: Some("bps".to_string()),
                    ..IntegerTypeNode::be(U16)
                },
            ),
        ] {
            assert_eq!(IntegerTypeNode::from_meta(&meta).unwrap(), expected);
        }
    }

    #[test]
    fn amount_display() {
        let display = || {
            let display = AmountNumberDisplayNode::new(
                IntegerValueNode::new(9u64),
                StringValueNode::new("SOL"),
            );
            Box::new(Some(NumberDisplayNode::Amount(display)))
        };
        for (meta, expected) in [
            (
                syn::parse_quote! { integer(u64, display = amount(decimals = 9, unit = "SOL")) },
                IntegerTypeNode {
                    display: display(),
                    ..IntegerTypeNode::le(U64)
                },
            ),
            (
                syn::parse_quote! { integer(u64, unit = "lamports", display = amount(decimals = 9, unit = "SOL")) },
                IntegerTypeNode {
                    unit: Some("lamports".to_string()),
                    display: display(),
                    ..IntegerTypeNode::le(U64)
                },
            ),
        ] {
            assert_eq!(IntegerTypeNode::from_meta(&meta).unwrap(), expected);
        }
    }

    #[test]
    fn unit_display() {
        for (meta, expected) in [
            (
                syn::parse_quote! { integer(u64, display = unit("lamports")) },
                UnitNumberDisplayNode::new(StringValueNode::new("lamports")),
            ),
            (
                syn::parse_quote! { integer(u64, display = unit(public_key("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"))) },
                UnitNumberDisplayNode::new(PublicKeyValueNode::new(
                    "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
                )),
            ),
        ] {
            assert_eq!(
                IntegerTypeNode::from_meta(&meta).unwrap(),
                IntegerTypeNode {
                    display: Box::new(Some(NumberDisplayNode::Unit(expected))),
                    ..IntegerTypeNode::le(U64)
                }
            );
        }
    }

    #[test]
    fn errors() {
        for (meta, expected) in [
            (syn::parse_quote! { integer(le) }, "format is missing"),
            (
                syn::parse_quote! { integer(u8, u16) },
                "format is already set",
            ),
            (
                syn::parse_quote! { integer(le, be) },
                "endian is already set",
            ),
            (
                syn::parse_quote! { integer(u64, display = amount(decimals = 9), display = amount(decimals = 6)) },
                "display is already set",
            ),
            (
                syn::parse_quote! { integer(u64, display = amount(unit = "SOL")) },
                "decimals is missing, use amount(decimals = ..., unit = ...) or unit(...)",
            ),
            (
                syn::parse_quote! { integer(u16, le, unknown) },
                "unrecognized attribute",
            ),
            (
                syn::parse_quote! { integer(u16, le, unknown = 42) },
                "unrecognized attribute",
            ),
            (
                syn::parse_quote! { integer(u16 = ?what?, le) },
                "unrecognized attribute",
            ),
            (
                syn::parse_quote! { integer(u64, unit = "slots", unit = "bps") },
                "unit is already set",
            ),
            (
                syn::parse_quote! { integer(u64, unit = slots) },
                "expected a string",
            ),
            (
                syn::parse_quote! { integer(u64, "slots") },
                "expected a path",
            ),
            (syn::parse_quote! { integer(f64) }, "unrecognized attribute"),
            (syn::parse_quote! { integer(42) }, "expected a path"),
        ] {
            let error = IntegerTypeNode::from_meta(&meta).unwrap_err();
            assert_eq!(error.to_string(), expected);
        }
    }
}
