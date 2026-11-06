use crate::utils::{
    hoist_integer_transforms, parse_encoding_integer, parse_ticks_per_second, FromMeta, SetOnce,
};
use codama_nodes::{DateTimeTypeNode, IntegerTypeNode};
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for DateTimeTypeNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        let pl = meta.assert_directive("date_time")?.as_path_list()?;
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
            DateTimeTypeNode {
                ticks_per_second: ticks_per_second.option(),
                ..DateTimeTypeNode::new(number)
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
        IntegerFormat::{I64, U64},
        PreOffsetTransformNode,
    };

    #[test]
    fn parses() {
        for (meta, expected) in [
            (
                syn::parse_quote! { date_time(integer(u64)) },
                DateTimeTypeNode::new(IntegerTypeNode::le(U64)),
            ),
            (
                syn::parse_quote! { date_time(number = integer(u64)) },
                DateTimeTypeNode::new(IntegerTypeNode::le(U64)),
            ),
            (
                syn::parse_quote! { date_time(integer(i64), ticks_per_second = 1000) },
                DateTimeTypeNode {
                    ticks_per_second: Some(1000),
                    ..DateTimeTypeNode::new(IntegerTypeNode::le(I64))
                },
            ),
            (
                syn::parse_quote! { date_time(ticks_per_second = 1_000_000_000, number = integer(i64, be)) },
                DateTimeTypeNode {
                    ticks_per_second: Some(1_000_000_000),
                    ..DateTimeTypeNode::new(IntegerTypeNode::be(I64))
                },
            ),
        ] {
            assert_eq!(DateTimeTypeNode::from_meta(&meta).unwrap(), expected);
        }
    }

    #[test]
    fn nested_number() {
        for (meta, expected) in [
            (
                syn::parse_quote! { date_time(fixed_size(integer(u64), 8)) },
                DateTimeTypeNode::new(IntegerTypeNode::le(U64))
                    .add_transform(FixedSizeTransformNode::new(8)),
            ),
            (
                syn::parse_quote! { date_time(number = fixed_size(integer(u64), 8)) },
                DateTimeTypeNode::new(IntegerTypeNode::le(U64))
                    .add_transform(FixedSizeTransformNode::new(8)),
            ),
            (
                syn::parse_quote! { date_time(pre_offset(fixed_size(integer(u64), 8), 4, relative)) },
                DateTimeTypeNode::new(IntegerTypeNode::le(U64))
                    .add_transform(FixedSizeTransformNode::new(8))
                    .add_transform(PreOffsetTransformNode::relative(4)),
            ),
        ] {
            assert_eq!(DateTimeTypeNode::from_meta(&meta).unwrap(), expected);
        }
    }

    #[test]
    fn wrapped() {
        assert_type!(
            { fixed_size(date_time(fixed_size(integer(u64), 8)), 16) },
            DateTimeTypeNode::new(IntegerTypeNode::le(U64))
                .add_transform(FixedSizeTransformNode::new(8))
                .add_transform(FixedSizeTransformNode::new(16))
                .into()
        );
    }

    #[test]
    fn errors() {
        for (meta, expected) in [
            (
                syn::parse_quote! { date_time(string) },
                "number must be an IntegerTypeNode",
            ),
            (
                syn::parse_quote! { date_time(fixed_size(string, 4)) },
                "number must be an IntegerTypeNode",
            ),
            (syn::parse_quote! { date_time() }, "number is missing"),
            (
                syn::parse_quote! { date_time(foo = 42) },
                "unrecognized attribute",
            ),
            (
                syn::parse_quote! { date_time(number(u64)) },
                "number(...) is removed in v2, use integer(...) or float(...)",
            ),
            (
                syn::parse_quote! { date_time(integer(i64, unit = "s")) },
                "number must not have a unit or a display",
            ),
            (
                syn::parse_quote! { date_time(number = integer(i64, display = amount(decimals = 3))) },
                "number must not have a unit or a display",
            ),
            (
                syn::parse_quote! { date_time(integer(i64), ticks_per_second = 0) },
                "ticks_per_second must be greater than zero",
            ),
            (
                syn::parse_quote! { date_time(integer(i64), ticks_per_second = 1000, ticks_per_second = 1) },
                "ticks_per_second is already set",
            ),
            (
                syn::parse_quote! { date_time(integer(i64), ticks_per_second = "1000") },
                "expected an unsigned integer",
            ),
            (
                syn::parse_quote! { date_time(integer(i64), ticks_per_second = 4294967296) },
                "number too large to fit in target type",
            ),
            (
                syn::parse_quote! { date_time(integer(i64), 1000) },
                "unrecognized attribute",
            ),
        ] {
            let error = DateTimeTypeNode::from_meta(&meta).unwrap_err();
            assert_eq!(error.to_string(), expected);
        }
    }
}
