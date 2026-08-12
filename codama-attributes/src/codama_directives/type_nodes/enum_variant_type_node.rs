use crate::utils::{FromMeta, SetOnce};
use codama_nodes::{
    CamelCaseString, EnumEmptyVariantTypeNode, EnumStructVariantTypeNode, EnumTupleVariantTypeNode,
    EnumVariantTypeNode, NestedTypeNode, TypeNode,
};
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for EnumVariantTypeNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        let pl = meta.assert_directive("variant")?.as_path_list()?;
        let mut name: SetOnce<CamelCaseString> = SetOnce::new("name");
        let mut discriminator: SetOnce<u32> = SetOnce::new("discriminator");
        let mut payload: SetOnce<TypeNode> = SetOnce::new("payload");

        pl.each(|ref meta| match meta.path_str().as_str() {
            "name" => name.set(meta.as_value()?.as_expr()?.as_string()?.into(), meta),
            "discriminator" => {
                discriminator.set(meta.as_value()?.as_expr()?.as_unsigned_integer()?, meta)
            }
            _ => {
                // The payload decides the variant kind: `struct(...)` or
                // `tuple(...)`, and no payload at all means an empty variant.
                if meta.is_path_or_list() {
                    return payload.set(TypeNode::from_meta(meta)?, meta);
                }
                if let Ok(value) = meta.as_expr().and_then(|expr| expr.as_string()) {
                    return name.set(value.into(), meta);
                }
                Err(meta.error("unrecognized attribute"))
            }
        })?;

        let name = name.take(meta)?;
        let discriminator = discriminator.option();

        match payload.option() {
            None => Ok(EnumEmptyVariantTypeNode {
                name,
                discriminator,
                display: None,
            }
            .into()),
            Some(TypeNode::Struct(node)) => Ok(EnumStructVariantTypeNode {
                name,
                discriminator,
                r#struct: NestedTypeNode::Value(node),
                display: None,
            }
            .into()),
            Some(TypeNode::Tuple(node)) => Ok(EnumTupleVariantTypeNode {
                name,
                discriminator,
                tuple: NestedTypeNode::Value(node),
                display: None,
            }
            .into()),
            Some(_) => Err(meta.error("a variant payload must be a struct or a tuple")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use codama_nodes::{
        NumberFormat::U8, NumberTypeNode, StructFieldTypeNode, StructTypeNode, TupleTypeNode,
    };

    fn parse(meta: Meta) -> syn::Result<EnumVariantTypeNode> {
        EnumVariantTypeNode::from_meta(&meta)
    }

    #[test]
    fn empty_variant() {
        assert_eq!(
            parse(syn::parse_quote! { variant("pending") }).unwrap(),
            EnumEmptyVariantTypeNode::new("pending").into()
        );
    }

    #[test]
    fn struct_variant() {
        assert_eq!(
            parse(syn::parse_quote! { variant("point", struct(field("x", number(u8)))) }).unwrap(),
            EnumStructVariantTypeNode::new(
                "point",
                StructTypeNode::new(vec![StructFieldTypeNode::new("x", NumberTypeNode::le(U8))])
            )
            .into()
        );
    }

    #[test]
    fn tuple_variant() {
        assert_eq!(
            parse(syn::parse_quote! { variant("pair", tuple(number(u8))) }).unwrap(),
            EnumTupleVariantTypeNode::new(
                "pair",
                TupleTypeNode::new(vec![NumberTypeNode::le(U8).into()])
            )
            .into()
        );
    }

    #[test]
    fn explicit_name() {
        assert_eq!(
            parse(syn::parse_quote! { variant(name = "pending") }).unwrap(),
            EnumEmptyVariantTypeNode::new("pending").into()
        );
    }

    #[test]
    fn discriminator() {
        let node = parse(syn::parse_quote! { variant("pending", discriminator = 3) }).unwrap();
        assert_eq!(
            node,
            EnumEmptyVariantTypeNode {
                name: "pending".into(),
                discriminator: Some(3),
                display: None,
            }
            .into()
        );
    }

    #[test]
    fn name_missing() {
        let error = parse(syn::parse_quote! { variant() }).unwrap_err();
        assert_eq!(error.to_string(), "name is missing");
    }

    #[test]
    fn invalid_payload() {
        let error = parse(syn::parse_quote! { variant("a", number(u8)) }).unwrap_err();
        assert_eq!(
            error.to_string(),
            "a variant payload must be a struct or a tuple"
        );
    }

    #[test]
    fn unrecognized_attribute() {
        let error = parse(syn::parse_quote! { variant("a", foo = 42) }).unwrap_err();
        assert_eq!(error.to_string(), "unrecognized attribute");
    }
}
