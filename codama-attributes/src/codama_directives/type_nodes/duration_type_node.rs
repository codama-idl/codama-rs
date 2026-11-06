use crate::utils::{
    hoist_integer_transforms, parse_encoding_integer, parse_ticks_per_second, FromMeta, SetOnce,
};
use codama_nodes::{DurationTypeNode, IntegerTypeNode};
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for DurationTypeNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        let pl = meta.assert_directive("duration")?.as_path_list()?;
        let mut number: SetOnce<IntegerTypeNode> = SetOnce::new("number");
        let mut ticks_per_second: SetOnce<u32> = SetOnce::new("ticks_per_second");

        pl.each(|ref meta| {
            // Lists and paths go to the type parser first, so `number(u64)` gets its hint, not the `number =` field.
            if meta.is_path_or_list() {
                return number.set(parse_encoding_integer(meta)?, meta);
            }
            match meta.path_str().as_str() {
                "number" => number.set(parse_encoding_integer(meta.as_value()?)?, meta),
                "ticks_per_second" => ticks_per_second.set(parse_ticks_per_second(meta)?, meta),
                _ => Err(meta.error("unrecognized attribute")),
            }
        })?;

        Ok(hoist_integer_transforms(number.take(meta)?, |number| {
            DurationTypeNode {
                ticks_per_second: ticks_per_second.option(),
                ..DurationTypeNode::new(number)
            }
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_type;
    use codama_nodes::{
        FixedSizeTransformNode, HasTransforms,
        IntegerFormat::{I64, U32, U64},
        PreOffsetTransformNode,
    };

    #[test]
    fn parses() {
        for (meta, expected) in [
            (
                syn::parse_quote! { duration(integer(u32)) },
                DurationTypeNode::new(IntegerTypeNode::le(U32)),
            ),
            (
                syn::parse_quote! { duration(number = integer(u32)) },
                DurationTypeNode::new(IntegerTypeNode::le(U32)),
            ),
            (
                syn::parse_quote! { duration(integer(u64), ticks_per_second = 1000) },
                DurationTypeNode {
                    ticks_per_second: Some(1000),
                    ..DurationTypeNode::new(IntegerTypeNode::le(U64))
                },
            ),
            (
                syn::parse_quote! { duration(ticks_per_second = 1_000_000, number = integer(i64, be)) },
                DurationTypeNode {
                    ticks_per_second: Some(1_000_000),
                    ..DurationTypeNode::new(IntegerTypeNode::be(I64))
                },
            ),
        ] {
            assert_eq!(DurationTypeNode::from_meta(&meta).unwrap(), expected);
        }
    }

    #[test]
    fn nested_number() {
        for (meta, expected) in [
            (
                syn::parse_quote! { duration(fixed_size(integer(u64), 8)) },
                DurationTypeNode::new(IntegerTypeNode::le(U64))
                    .add_transform(FixedSizeTransformNode::new(8)),
            ),
            (
                syn::parse_quote! { duration(number = fixed_size(integer(u64), 8), ticks_per_second = 1000) },
                DurationTypeNode {
                    ticks_per_second: Some(1000),
                    ..DurationTypeNode::new(IntegerTypeNode::le(U64))
                }
                .add_transform(FixedSizeTransformNode::new(8)),
            ),
            (
                syn::parse_quote! { duration(pre_offset(fixed_size(integer(u64), 8), 4, relative)) },
                DurationTypeNode::new(IntegerTypeNode::le(U64))
                    .add_transform(FixedSizeTransformNode::new(8))
                    .add_transform(PreOffsetTransformNode::relative(4)),
            ),
        ] {
            assert_eq!(DurationTypeNode::from_meta(&meta).unwrap(), expected);
        }
    }

    #[test]
    fn wrapped() {
        assert_type!(
            { fixed_size(duration(fixed_size(integer(u64), 8)), 16) },
            DurationTypeNode::new(IntegerTypeNode::le(U64))
                .add_transform(FixedSizeTransformNode::new(8))
                .add_transform(FixedSizeTransformNode::new(16))
                .into()
        );
    }

    #[test]
    fn errors() {
        for (meta, expected) in [
            (
                syn::parse_quote! { duration(string) },
                "number must be an IntegerTypeNode",
            ),
            (
                syn::parse_quote! { duration(float(f64)) },
                "number must be an IntegerTypeNode",
            ),
            (
                syn::parse_quote! { duration(fixed_size(string, 4)) },
                "number must be an IntegerTypeNode",
            ),
            (
                syn::parse_quote! { duration(integer(u64, unit = "ms")) },
                "number must not have a unit or a display",
            ),
            (
                syn::parse_quote! { duration(number = integer(u64, display = amount(decimals = 3))) },
                "number must not have a unit or a display",
            ),
            (syn::parse_quote! { duration() }, "number is missing"),
            (
                syn::parse_quote! { duration(ticks_per_second = 1000) },
                "number is missing",
            ),
            (
                syn::parse_quote! { duration(integer(u32), integer(u64)) },
                "number is already set",
            ),
            (
                syn::parse_quote! { duration(integer(u64), ticks_per_second = 0) },
                "ticks_per_second must be greater than zero",
            ),
            (
                syn::parse_quote! { duration(integer(u64), ticks_per_second = 1000, ticks_per_second = 1) },
                "ticks_per_second is already set",
            ),
            (
                syn::parse_quote! { duration(integer(u64), ticks_per_second = "1000") },
                "expected an unsigned integer",
            ),
            (
                syn::parse_quote! { duration(integer(u64), 1000) },
                "unrecognized attribute",
            ),
            (
                syn::parse_quote! { duration(foo = 42) },
                "unrecognized attribute",
            ),
            (
                syn::parse_quote! { duration(number(u64)) },
                "number(...) is removed in v2, use integer(...) or float(...)",
            ),
        ] {
            let error = DurationTypeNode::from_meta(&meta).unwrap_err();
            assert_eq!(error.to_string(), expected);
        }
    }
}
