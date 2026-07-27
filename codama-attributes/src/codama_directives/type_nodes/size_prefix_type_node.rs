use crate::utils::{FromMeta, SetOnce};
use codama_nodes::{NestedTypeNode, NumberTypeNode, SizePrefixTypeNode, TypeNode};
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for SizePrefixTypeNode<TypeNode> {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        let pl = meta.assert_directive("size_prefix")?.as_path_list()?;
        let mut r#type: SetOnce<TypeNode> = SetOnce::new("type");
        let mut prefix: SetOnce<NestedTypeNode<NumberTypeNode>> = SetOnce::new("prefix");

        pl.each(|ref meta| match meta.path_str().as_str() {
            "type" => r#type.set(TypeNode::from_meta(meta.as_value()?)?, meta),
            "prefix" => prefix.set(parse_prefix(meta.as_value()?)?, meta),
            _ => {
                // Both are type nodes, so positional args fill `type` then `prefix`.
                if meta.is_path_or_list() {
                    return match r#type.is_set() {
                        false => r#type.set(TypeNode::from_meta(meta)?, meta),
                        true => prefix.set(parse_prefix(meta)?, meta),
                    };
                }
                Err(meta.error("unrecognized attribute"))
            }
        })?;

        Ok(Self::new(r#type.take(meta)?, prefix.take(meta)?))
    }
}

fn parse_prefix(meta: &Meta) -> syn::Result<NestedTypeNode<NumberTypeNode>> {
    let node = TypeNode::from_meta(meta)?;
    NestedTypeNode::<NumberTypeNode>::try_from(node)
        .map_err(|_| meta.error("prefix must be a NumberTypeNode"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_type, assert_type_err};
    use codama_nodes::{
        BytesTypeNode, FixedSizeTypeNode,
        NumberFormat::{U16, U32, U8},
        OptionTypeNode, StringTypeNode,
    };

    fn parse_node(meta: Meta) -> SizePrefixTypeNode<TypeNode> {
        SizePrefixTypeNode::<TypeNode>::from_meta(&meta).unwrap()
    }

    #[test]
    fn parsed_fields() {
        let node = parse_node(syn::parse_quote! { size_prefix(string, number(u32)) });
        assert_eq!(*node.r#type, TypeNode::String(StringTypeNode::utf8()));
        assert_eq!(*node.prefix, NestedTypeNode::Value(NumberTypeNode::le(U32)));

        let node = parse_node(syn::parse_quote! { size_prefix(bytes, number(u16, be)) });
        assert_eq!(*node.r#type, TypeNode::Bytes(BytesTypeNode::new()));
        assert_eq!(*node.prefix, NestedTypeNode::Value(NumberTypeNode::be(U16)));
    }

    #[test]
    fn equivalent_syntaxes() {
        let expected = SizePrefixTypeNode::new(StringTypeNode::utf8(), NumberTypeNode::le(U32));
        let positional = parse_node(syn::parse_quote! { size_prefix(string, number(u32)) });
        let named_prefix =
            parse_node(syn::parse_quote! { size_prefix(string, prefix = number(u32)) });
        let fully_explicit =
            parse_node(syn::parse_quote! { size_prefix(type = string, prefix = number(u32)) });

        assert_eq!(positional, expected);
        assert_eq!(named_prefix, expected);
        assert_eq!(fully_explicit, expected);
        assert_eq!(positional, named_prefix);
        assert_eq!(named_prefix, fully_explicit);
    }

    #[test]
    fn explicit() {
        assert_type!(
            { size_prefix(type = string, prefix = number(u32)) },
            SizePrefixTypeNode::new(StringTypeNode::utf8(), NumberTypeNode::le(U32)).into()
        );
        assert_type!(
            { size_prefix(prefix = number(u32), type = string) },
            SizePrefixTypeNode::new(StringTypeNode::utf8(), NumberTypeNode::le(U32)).into()
        );
        assert_type!(
            { size_prefix(prefix = number(u32), string) },
            SizePrefixTypeNode::new(StringTypeNode::utf8(), NumberTypeNode::le(U32)).into()
        );
    }

    #[test]
    fn nested() {
        assert_type!(
            { option(size_prefix(string, number(u8))) },
            OptionTypeNode::new(SizePrefixTypeNode::new(
                StringTypeNode::utf8(),
                NumberTypeNode::le(U8)
            ))
            .into()
        );

        let meta: Meta = syn::parse_quote! { option(size_prefix(string, number(u8))) };
        let node = OptionTypeNode::from_meta(&meta).unwrap();
        assert_eq!(
            *node.item,
            TypeNode::SizePrefix(SizePrefixTypeNode::new(
                StringTypeNode::utf8(),
                NumberTypeNode::le(U8)
            ))
        );
    }

    #[test]
    fn nested_prefix() {
        assert_type!(
            { size_prefix(string, fixed_size(number(u32), 4)) },
            SizePrefixTypeNode::new(
                StringTypeNode::utf8(),
                FixedSizeTypeNode::<NestedTypeNode<NumberTypeNode>>::new(
                    NumberTypeNode::le(U32),
                    4
                )
            )
            .into()
        );
    }

    #[test]
    fn invalid_prefix() {
        assert_type_err!(
            { size_prefix(string, public_key) },
            "prefix must be a NumberTypeNode"
        );
        assert_type_err!(
            { size_prefix(type = string, prefix = public_key) },
            "prefix must be a NumberTypeNode"
        );
    }

    #[test]
    fn type_missing() {
        assert_type_err!({ size_prefix(prefix = number(u32)) }, "type is missing");
    }

    #[test]
    fn prefix_missing() {
        assert_type_err!({ size_prefix(string) }, "prefix is missing");
    }

    #[test]
    fn already_set() {
        assert_type_err!(
            { size_prefix(type = string, type = bytes) },
            "type is already set"
        );
        assert_type_err!(
            { size_prefix(string, number(u32), number(u8)) },
            "prefix is already set"
        );
    }

    #[test]
    fn unrecognized_type() {
        assert_type_err!(
            { size_prefix(unrecognized, number(u32)) },
            "unrecognized type"
        );
    }

    #[test]
    fn unrecognized_attribute() {
        assert_type_err!({ size_prefix(string, foo = 42) }, "unrecognized attribute");
    }
}
