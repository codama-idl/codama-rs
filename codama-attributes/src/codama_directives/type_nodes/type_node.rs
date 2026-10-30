use crate::{
    utils::{removed_syntax, FromMeta},
    TypeDirectiveNode,
};
use codama_nodes::{
    ArrayTypeNode, BooleanTypeNode, BytesTypeNode, DateTimeTypeNode, DurationTypeNode,
    EnumTypeNode, EnumVariantTypeNode, FixedPointTypeNode, FloatTypeNode, IntegerTypeNode,
    MapTypeNode, OptionTypeNode, PublicKeyTypeNode, RegisteredTypeNode, RemainderOptionTypeNode,
    SetTypeNode, StringTypeNode, StructFieldTypeNode, StructTypeNode, TupleTypeNode, TypeNode,
    ZeroableOptionTypeNode,
};
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for RegisteredTypeNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        match meta.path_str().as_str() {
            "amount" => removed_syntax(
                meta,
                "amount(...)",
                "fixed_point(...) or integer(..., unit = ...)",
            ),
            "array" => ArrayTypeNode::from_meta(meta).map(Self::from),
            "boolean" => BooleanTypeNode::from_meta(meta).map(Self::from),
            "bytes" => BytesTypeNode::from_meta(meta).map(Self::from),
            "date_time" => DateTimeTypeNode::from_meta(meta).map(Self::from),
            "duration" => DurationTypeNode::from_meta(meta).map(Self::from),
            "enum" => EnumTypeNode::from_meta(meta).map(Self::from),
            "field" => StructFieldTypeNode::from_meta(meta).map(Self::from),
            "fixed_point" => FixedPointTypeNode::from_meta(meta).map(Self::from),
            "float" => FloatTypeNode::from_meta(meta).map(Self::from),
            "integer" => IntegerTypeNode::from_meta(meta).map(Self::from),
            "map" => MapTypeNode::from_meta(meta).map(Self::from),
            "number" => removed_syntax(meta, "number(...)", "integer(...) or float(...)"),
            "option" => OptionTypeNode::from_meta(meta).map(Self::from),
            "public_key" => PublicKeyTypeNode::from_meta(meta).map(Self::from),
            "remainder_option" => RemainderOptionTypeNode::from_meta(meta).map(Self::from),
            "set" => SetTypeNode::from_meta(meta).map(Self::from),
            "sol_amount" => removed_syntax(
                meta,
                "sol_amount(...)",
                "fixed_point(..., scale = 9, unit = \"SOL\")",
            ),
            "string" => StringTypeNode::from_meta(meta).map(Self::from),
            "struct" => StructTypeNode::from_meta(meta).map(Self::from),
            "tuple" => TupleTypeNode::from_meta(meta).map(Self::from),
            "variant" => EnumVariantTypeNode::from_meta(meta).map(Self::from),
            "zeroable_option" => ZeroableOptionTypeNode::from_meta(meta).map(Self::from),
            _ => Err(meta.error("unrecognized type")),
        }
    }
}

impl FromMeta for TypeNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        Self::try_from(TypeDirectiveNode::from_meta(meta)?)
            .map_err(|_| meta.error("unrecognized type"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use codama_nodes::{
        Base, FixedSizeTransformNode,
        FloatFormat::F64,
        HasTransforms,
        IntegerFormat::{I64, U128, U16, U64},
    };

    #[test]
    fn removed_number() {
        for meta in [
            syn::parse_quote! { number(u64) },
            syn::parse_quote! { number(f64, be) },
        ] {
            let error = TypeNode::from_meta(&meta).unwrap_err();
            assert_eq!(
                error.to_string(),
                "number(...) is removed in v2, use integer(...) or float(...)"
            );
        }
    }

    #[test]
    fn removed_amount() {
        for meta in [
            syn::parse_quote! { amount(number(u64), 6, "USDC") },
            syn::parse_quote! { amount(number = number(u64), decimals = 0, unit = "bytes") },
        ] {
            let error = TypeNode::from_meta(&meta).unwrap_err();
            assert_eq!(
                error.to_string(),
                "amount(...) is removed in v2, use fixed_point(...) or integer(..., unit = ...)"
            );
        }
    }

    #[test]
    fn removed_sol_amount() {
        let meta: Meta = syn::parse_quote! { sol_amount(number(u64)) };
        let error = TypeNode::from_meta(&meta).unwrap_err();
        assert_eq!(
            error.to_string(),
            "sol_amount(...) is removed in v2, use fixed_point(..., scale = 9, unit = \"SOL\")"
        );
    }

    #[test]
    fn fixed_point_duration_and_units_in_nested_positions() {
        let fixed_point = || FixedPointTypeNode {
            unit: Some("USDC".to_string()),
            ..FixedPointTypeNode::new(IntegerTypeNode::le(U64), 6)
        };
        let duration = || DurationTypeNode {
            ticks_per_second: Some(1000),
            ..DurationTypeNode::new(IntegerTypeNode::le(U64))
        };
        for (meta, expected) in [
            (
                syn::parse_quote! { option(fixed_point(integer(u64), scale = 6, unit = "USDC")) },
                TypeNode::from(OptionTypeNode::new(fixed_point())),
            ),
            (
                syn::parse_quote! { array(duration(integer(u64), ticks_per_second = 1000), 3) },
                ArrayTypeNode::fixed(duration(), 3).into(),
            ),
            (
                syn::parse_quote! { map(string, fixed_point(integer(u128), scale = 64, base = 2), 2) },
                MapTypeNode::fixed(
                    StringTypeNode::utf8(),
                    FixedPointTypeNode {
                        base: Some(Base::Base2),
                        ..FixedPointTypeNode::new(IntegerTypeNode::le(U128), 64)
                    },
                    2,
                )
                .into(),
            ),
            (
                syn::parse_quote! { struct(field("at", date_time(integer(i64), ticks_per_second = 1000))) },
                StructTypeNode::new(vec![StructFieldTypeNode::new(
                    "at",
                    DateTimeTypeNode {
                        ticks_per_second: Some(1000),
                        ..DateTimeTypeNode::new(IntegerTypeNode::le(I64))
                    },
                )])
                .into(),
            ),
            (
                syn::parse_quote! { enum(variant("timeout", duration(integer(u64), ticks_per_second = 1000))) },
                EnumTypeNode::new(vec![EnumVariantTypeNode::with_data("timeout", duration())])
                    .into(),
            ),
            (
                syn::parse_quote! { tuple(integer(u64, unit = "slots"), float(f64, unit = "USD")) },
                TupleTypeNode::new(vec![
                    IntegerTypeNode {
                        unit: Some("slots".to_string()),
                        ..IntegerTypeNode::le(U64)
                    }
                    .into(),
                    FloatTypeNode {
                        unit: Some("USD".to_string()),
                        ..FloatTypeNode::le(F64)
                    }
                    .into(),
                ])
                .into(),
            ),
            (
                syn::parse_quote! { set(integer(u16, unit = "bps"), 2) },
                SetTypeNode::fixed(
                    IntegerTypeNode {
                        unit: Some("bps".to_string()),
                        ..IntegerTypeNode::le(U16)
                    },
                    2,
                )
                .into(),
            ),
            (
                syn::parse_quote! { fixed_size(fixed_point(integer(u64), scale = 6, unit = "USDC"), 16) },
                fixed_point()
                    .add_transform(FixedSizeTransformNode::new(16))
                    .into(),
            ),
            (
                syn::parse_quote! { fixed_point(fixed_size(integer(u64), 8), scale = 6, unit = "USDC") },
                fixed_point()
                    .add_transform(FixedSizeTransformNode::new(8))
                    .into(),
            ),
            (
                syn::parse_quote! { fixed_size(duration(fixed_size(integer(u64), 8), ticks_per_second = 1000), 16) },
                duration()
                    .add_transform(FixedSizeTransformNode::new(8))
                    .add_transform(FixedSizeTransformNode::new(16))
                    .into(),
            ),
        ] {
            assert_eq!(TypeNode::from_meta(&meta).unwrap(), expected);
        }
    }
}
