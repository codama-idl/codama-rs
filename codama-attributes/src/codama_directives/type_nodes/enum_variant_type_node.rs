use crate::utils::{FromMeta, SetOnce};
use codama_nodes::{CamelCaseString, EnumVariantTypeNode, TypeNode};
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
        let mut node = match payload.option() {
            Some(data) => EnumVariantTypeNode::with_data(name, data),
            None => EnumVariantTypeNode::new(name),
        };
        node.discriminator = discriminator.option();
        Ok(node)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use codama_nodes::{
        ArrayTypeNode, DefinedTypeLinkNode, FixedCountNode,
        IntegerFormat::{U64, U8},
        IntegerTypeNode, PublicKeyTypeNode, StructFieldTypeNode, StructTypeNode, TupleTypeNode,
    };

    fn parse(meta: Meta) -> syn::Result<EnumVariantTypeNode> {
        EnumVariantTypeNode::from_meta(&meta)
    }

    #[test]
    fn empty_variant() {
        assert_eq!(
            parse(syn::parse_quote! { variant("pending") }).unwrap(),
            EnumVariantTypeNode::new("pending")
        );
    }

    #[test]
    fn struct_variant() {
        assert_eq!(
            parse(syn::parse_quote! { variant("point", struct(field("x", integer(u8)))) }).unwrap(),
            EnumVariantTypeNode::with_data(
                "point",
                StructTypeNode::new(vec![StructFieldTypeNode::new("x", IntegerTypeNode::le(U8))])
            )
        );
    }

    #[test]
    fn tuple_variant() {
        assert_eq!(
            parse(syn::parse_quote! { variant("pair", tuple(integer(u8))) }).unwrap(),
            EnumVariantTypeNode::with_data(
                "pair",
                TupleTypeNode::new(vec![IntegerTypeNode::le(U8).into()])
            )
        );
    }

    #[test]
    fn explicit_name() {
        assert_eq!(
            parse(syn::parse_quote! { variant(name = "pending") }).unwrap(),
            EnumVariantTypeNode::new("pending")
        );
    }

    #[test]
    fn discriminator() {
        let node = parse(syn::parse_quote! { variant("pending", discriminator = 3) }).unwrap();
        assert_eq!(
            node,
            EnumVariantTypeNode {
                discriminator: Some(3),
                ..EnumVariantTypeNode::new("pending")
            }
        );
    }

    #[test]
    fn name_missing() {
        let error = parse(syn::parse_quote! { variant() }).unwrap_err();
        assert_eq!(error.to_string(), "name is missing");
    }

    #[test]
    fn any_payload() {
        assert_eq!(
            parse(syn::parse_quote! { variant("amount", integer(u64)) }).unwrap(),
            EnumVariantTypeNode::with_data("amount", IntegerTypeNode::le(U64))
        );
        assert_eq!(
            parse(syn::parse_quote! { variant("owner", public_key) }).unwrap(),
            EnumVariantTypeNode::with_data("owner", PublicKeyTypeNode::new())
        );
        assert_eq!(
            parse(syn::parse_quote! { variant("items", array(integer(u8), 4)) }).unwrap(),
            EnumVariantTypeNode::with_data(
                "items",
                ArrayTypeNode::new(IntegerTypeNode::le(U8), FixedCountNode::new(4))
            )
        );
        assert_eq!(
            parse(syn::parse_quote! { variant("target", link("pubkey")) }).unwrap(),
            EnumVariantTypeNode::with_data("target", DefinedTypeLinkNode::new("pubkey"))
        );
    }

    #[test]
    fn unrecognized_attribute() {
        let error = parse(syn::parse_quote! { variant("a", foo = 42) }).unwrap_err();
        assert_eq!(error.to_string(), "unrecognized attribute");
    }
}
