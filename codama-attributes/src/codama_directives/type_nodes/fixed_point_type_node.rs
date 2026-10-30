use crate::utils::{
    hoist_integer_transforms, parse_encoding_integer, parse_unit_display, FromMeta, SetOnce,
};
use codama_nodes::{
    Base, FixedPointTypeNode, IntegerFormat, IntegerTypeNode, UnitNumberDisplayNode,
};
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for FixedPointTypeNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        let pl = meta.assert_directive("fixed_point")?.as_path_list()?;
        let mut number: SetOnce<IntegerTypeNode> = SetOnce::new("number");
        let mut scale: SetOnce<u32> = SetOnce::new("scale");
        let mut base: SetOnce<Base> = SetOnce::new("base");
        let mut unit: SetOnce<String> = SetOnce::new("unit");
        let mut display: SetOnce<UnitNumberDisplayNode> = SetOnce::new("display");

        pl.each(|ref meta| {
            // Lists and paths go to the type parser first, so `number(u64)` gets its hint, not the `number =` field.
            if meta.is_path_or_list() {
                return number.set(parse_number(meta)?, meta);
            }
            match meta.path_str().as_str() {
                "number" => number.set(parse_number(meta.as_value()?)?, meta),
                "scale" => {
                    let value = meta.as_value()?;
                    match value.as_expr()?.as_unsigned_integer()? {
                        0 => Err(value.error("scale must be greater than zero")),
                        value => scale.set(value, meta),
                    }
                }
                "base" => {
                    let value = meta.as_value()?;
                    match Base::try_from(value.as_expr()?.as_unsigned_integer::<u64>()?) {
                        Ok(parsed) => base.set(parsed, meta),
                        Err(_) => Err(value.error("base must be 2 or 10")),
                    }
                }
                "unit" => unit.set(meta.as_value()?.as_expr()?.as_string()?, meta),
                "display" => {
                    display.set(parse_unit_display(meta.as_value()?, "fixed points")?, meta)
                }
                _ => Err(meta.error("unrecognized attribute")),
            }
        })?;

        let (number, scale) = (number.take(meta)?, scale.take(meta)?);
        Ok(hoist_integer_transforms(number, |number| {
            FixedPointTypeNode {
                base: base.option(),
                unit: unit.option(),
                display: display.option(),
                ..FixedPointTypeNode::new(number, scale)
            }
        }))
    }
}

/// Parses the number of a fixed point, which needs a fixed-size format: a fixed point presupposes a fixed bit width.
fn parse_number(meta: &Meta) -> syn::Result<IntegerTypeNode> {
    let number = parse_encoding_integer(meta)?;
    match number.format {
        IntegerFormat::ShortU16 => Err(meta.error("number must use a fixed-size integer format")),
        _ => Ok(number),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_type;
    use codama_nodes::{
        FixedSizeTransformNode, HasTransforms,
        IntegerFormat::{I64, U128, U64},
        PreOffsetTransformNode, StringValueNode,
    };

    #[test]
    fn parses() {
        for (meta, expected) in [
            (
                syn::parse_quote! { fixed_point(integer(u64), scale = 6) },
                FixedPointTypeNode::new(IntegerTypeNode::le(U64), 6),
            ),
            (
                syn::parse_quote! { fixed_point(integer(u64), scale = 6, unit = "USDC") },
                FixedPointTypeNode {
                    unit: Some("USDC".to_string()),
                    ..FixedPointTypeNode::new(IntegerTypeNode::le(U64), 6)
                },
            ),
            (
                syn::parse_quote! { fixed_point(number = integer(u64), scale = 9, unit = "SOL") },
                FixedPointTypeNode {
                    unit: Some("SOL".to_string()),
                    ..FixedPointTypeNode::new(IntegerTypeNode::le(U64), 9)
                },
            ),
            (
                syn::parse_quote! { fixed_point(unit = "%", scale = 2, number = integer(i64, be)) },
                FixedPointTypeNode {
                    unit: Some("%".to_string()),
                    ..FixedPointTypeNode::new(IntegerTypeNode::be(I64), 2)
                },
            ),
            (
                syn::parse_quote! { fixed_point(integer(u128), base = 2, scale = 64) },
                FixedPointTypeNode {
                    base: Some(Base::Base2),
                    ..FixedPointTypeNode::new(IntegerTypeNode::le(U128), 64)
                },
            ),
            (
                syn::parse_quote! { fixed_point(integer(u64), scale = 2, base = 10) },
                FixedPointTypeNode {
                    base: Some(Base::Base10),
                    ..FixedPointTypeNode::new(IntegerTypeNode::le(U64), 2)
                },
            ),
        ] {
            assert_eq!(FixedPointTypeNode::from_meta(&meta).unwrap(), expected);
        }
    }

    #[test]
    fn unit_display() {
        let display = || Some(UnitNumberDisplayNode::new(StringValueNode::new("wSOL")));
        for (meta, expected) in [
            (
                syn::parse_quote! { fixed_point(integer(u64), scale = 9, display = unit("wSOL")) },
                FixedPointTypeNode {
                    display: display(),
                    ..FixedPointTypeNode::new(IntegerTypeNode::le(U64), 9)
                },
            ),
            (
                syn::parse_quote! { fixed_point(display = unit("wSOL"), unit = "SOL", scale = 9, number = integer(u64)) },
                FixedPointTypeNode {
                    unit: Some("SOL".to_string()),
                    display: display(),
                    ..FixedPointTypeNode::new(IntegerTypeNode::le(U64), 9)
                },
            ),
        ] {
            assert_eq!(FixedPointTypeNode::from_meta(&meta).unwrap(), expected);
        }
    }

    #[test]
    fn amount_display() {
        for meta in [
            syn::parse_quote! { fixed_point(integer(u64), scale = 9, display = amount(decimals = 9, unit = "SOL")) },
            syn::parse_quote! { fixed_point(integer(u64), scale = 9, display = amount(unit = "SOL")) },
        ] {
            let error = FixedPointTypeNode::from_meta(&meta).unwrap_err();
            assert_eq!(
                error.to_string(),
                "fixed points take a unit display, use unit(...)"
            );
        }
    }

    #[test]
    fn nested_number() {
        for (meta, expected) in [
            (
                syn::parse_quote! { fixed_point(fixed_size(integer(u64), 8), scale = 6) },
                FixedPointTypeNode::new(IntegerTypeNode::le(U64), 6)
                    .add_transform(FixedSizeTransformNode::new(8)),
            ),
            (
                syn::parse_quote! { fixed_point(number = fixed_size(integer(u64), 8), scale = 6, unit = "USDC") },
                FixedPointTypeNode {
                    unit: Some("USDC".to_string()),
                    ..FixedPointTypeNode::new(IntegerTypeNode::le(U64), 6)
                }
                .add_transform(FixedSizeTransformNode::new(8)),
            ),
            (
                syn::parse_quote! { fixed_point(pre_offset(fixed_size(integer(u64), 8), 4, relative), scale = 6) },
                FixedPointTypeNode::new(IntegerTypeNode::le(U64), 6)
                    .add_transform(FixedSizeTransformNode::new(8))
                    .add_transform(PreOffsetTransformNode::relative(4)),
            ),
        ] {
            assert_eq!(FixedPointTypeNode::from_meta(&meta).unwrap(), expected);
        }
    }

    #[test]
    fn wrapped() {
        assert_type!(
            { fixed_size(fixed_point(fixed_size(integer(u64), 8), scale = 6), 16) },
            FixedPointTypeNode::new(IntegerTypeNode::le(U64), 6)
                .add_transform(FixedSizeTransformNode::new(8))
                .add_transform(FixedSizeTransformNode::new(16))
                .into()
        );
    }

    #[test]
    fn errors() {
        for (meta, expected) in [
            (
                syn::parse_quote! { fixed_point(string, scale = 6) },
                "number must be an IntegerTypeNode",
            ),
            (
                syn::parse_quote! { fixed_point(float(f64), scale = 6) },
                "number must be an IntegerTypeNode",
            ),
            (
                syn::parse_quote! { fixed_point(integer(short_u16), scale = 2) },
                "number must use a fixed-size integer format",
            ),
            (
                syn::parse_quote! { fixed_point(integer(u64, unit = "USDC"), scale = 6) },
                "number must not have a unit or a display",
            ),
            (
                syn::parse_quote! { fixed_point(number = integer(u64, display = amount(decimals = 6)), scale = 6) },
                "number must not have a unit or a display",
            ),
            (
                syn::parse_quote! { fixed_point(scale = 6) },
                "number is missing",
            ),
            (
                syn::parse_quote! { fixed_point(integer(u64)) },
                "scale is missing",
            ),
            (
                syn::parse_quote! { fixed_point(integer(u64), scale = 0) },
                "scale must be greater than zero",
            ),
            (
                syn::parse_quote! { fixed_point(integer(u64), scale = 6, scale = 9) },
                "scale is already set",
            ),
            (
                syn::parse_quote! { fixed_point(integer(u64), scale = -6) },
                "expected an unsigned integer",
            ),
            (
                syn::parse_quote! { fixed_point(integer(u64), scale = 6, base = 16) },
                "base must be 2 or 10",
            ),
            (
                syn::parse_quote! { fixed_point(integer(u64), scale = 6, base = 2, base = 10) },
                "base is already set",
            ),
            (
                syn::parse_quote! { fixed_point(integer(u64), scale = 6, unit = USDC) },
                "expected a string",
            ),
            (
                syn::parse_quote! { fixed_point(integer(u64), scale = 6, unit = "USDC", unit = "SOL") },
                "unit is already set",
            ),
            (
                syn::parse_quote! { fixed_point(integer(u64), 6, "USDC") },
                "unrecognized attribute",
            ),
            (
                syn::parse_quote! { fixed_point(integer(u64), scale = 6, foo = 42) },
                "unrecognized attribute",
            ),
            (
                syn::parse_quote! { fixed_point(integer(u64), scale = 6, display = unit("A"), display = unit("B")) },
                "display is already set",
            ),
            (
                syn::parse_quote! { fixed_point(number(u64), scale = 6) },
                "number(...) is removed in v2, use integer(...) or float(...)",
            ),
        ] {
            let error = FixedPointTypeNode::from_meta(&meta).unwrap_err();
            assert_eq!(error.to_string(), expected);
        }
    }
}
