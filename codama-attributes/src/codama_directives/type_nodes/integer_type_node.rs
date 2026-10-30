use crate::utils::{FromMeta, SetOnce};
use codama_nodes::{Endianness, IntegerFormat, IntegerTypeNode, NumberDisplayNode};
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for IntegerTypeNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        let pl = meta.assert_directive("integer")?.as_path_list()?;
        let mut format = SetOnce::<IntegerFormat>::new("format");
        let mut endian = SetOnce::<Endianness>::new("endian").initial_value(Endianness::Le);
        let mut unit = SetOnce::<String>::new("unit");
        let mut display = SetOnce::<NumberDisplayNode>::new("display");

        pl.each(|ref meta| match meta.path_str().as_str() {
            "format" => {
                let path = meta.as_value()?.as_path()?;
                match IntegerFormat::try_from(path.to_string().as_str()) {
                    Ok(value) => format.set(value, meta),
                    Err(_) => Err(path.error("invalid format")),
                }
            }
            "endian" => {
                let path = meta.as_value()?.as_path()?;
                match Endianness::try_from(path.to_string()) {
                    Ok(value) => endian.set(value, meta),
                    _ => Err(path.error("invalid endian")),
                }
            }
            "unit" => unit.set(meta.as_value()?.as_expr()?.as_string()?, meta),
            "display" => display.set(NumberDisplayNode::from_meta(meta.as_value()?)?, meta),
            _ => {
                if let Ok(path) = meta.as_path() {
                    if let Ok(value) = IntegerFormat::try_from(path.to_string().as_str()) {
                        return format.set(value, meta);
                    }
                    if let Ok(value) = Endianness::try_from(path.to_string()) {
                        return endian.set(value, meta);
                    }
                }
                Err(meta.path()?.error("unrecognized attribute"))
            }
        })?;

        Ok(IntegerTypeNode {
            unit: unit.option(),
            display: Box::new(display.option()),
            ..IntegerTypeNode::new(format.take(meta)?, endian.take(meta)?)
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
