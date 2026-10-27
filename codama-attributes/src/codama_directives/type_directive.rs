use crate::{
    codama_directives::type_nodes::{
        fixed_size_type_node_from_meta, hidden_prefix_type_node_from_meta,
        hidden_suffix_type_node_from_meta, post_offset_type_node_from_meta,
        pre_offset_type_node_from_meta, sentinel_type_node_from_meta,
        size_prefix_type_node_from_meta,
    },
    utils::FromMeta,
    Attribute, CodamaAttribute, CodamaDirective, Resolvable,
};
use codama_errors::CodamaError;
use codama_nodes::{DefinedTypeLinkNode, Node, RegisteredTypeNode, TypeNode};
use codama_syn_helpers::{extensions::*, Meta};

#[derive(Debug, PartialEq)]
pub struct TypeDirective {
    pub node: Resolvable<TypeDirectiveNode>,
}

impl TypeDirective {
    pub fn parse(meta: &Meta) -> syn::Result<Self> {
        let value = meta.assert_directive("type")?.as_value()?;
        Ok(Self {
            node: Resolvable::<TypeDirectiveNode>::from_meta(value)?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, derive_more::From)]
pub enum TypeDirectiveNode {
    Registered(RegisteredTypeNode),
    Link(DefinedTypeLinkNode),
}

impl FromMeta for TypeDirectiveNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        // Wrapper keywords are DSL sugar with no v2 node. Each returns its inner type plus a transform.
        // That type can be a link, so they are dispatched here, not in `RegisteredTypeNode::from_meta`.
        let node = match meta.path_str().as_str() {
            "link" => return DefinedTypeLinkNode::from_meta(meta).map(Self::from),
            "fixed_size" => fixed_size_type_node_from_meta(meta)?,
            "hidden_prefix" => hidden_prefix_type_node_from_meta(meta)?,
            "hidden_suffix" => hidden_suffix_type_node_from_meta(meta)?,
            "post_offset" => post_offset_type_node_from_meta(meta)?,
            "pre_offset" => pre_offset_type_node_from_meta(meta)?,
            "sentinel" => sentinel_type_node_from_meta(meta)?,
            "size_prefix" => size_prefix_type_node_from_meta(meta)?,
            _ => return RegisteredTypeNode::from_meta(meta).map(Self::from),
        };
        Self::try_from(node).map_err(|e| meta.error(e.to_string()))
    }
}

/// A wrapper keyword returns its inner type with a transform, so the result can be a link.
impl TryFrom<TypeNode> for TypeDirectiveNode {
    type Error = CodamaError;

    fn try_from(node: TypeNode) -> Result<Self, Self::Error> {
        match node {
            TypeNode::Link(node) => Ok(Self::Link(node)),
            node => RegisteredTypeNode::try_from(node).map(Self::Registered),
        }
    }
}

impl From<TypeDirectiveNode> for Node {
    fn from(node: TypeDirectiveNode) -> Self {
        match node {
            TypeDirectiveNode::Registered(node) => Self::Type(node),
            TypeDirectiveNode::Link(node) => node.into(),
        }
    }
}

impl TryFrom<TypeDirectiveNode> for TypeNode {
    type Error = CodamaError;

    fn try_from(node: TypeDirectiveNode) -> Result<Self, Self::Error> {
        match node {
            TypeDirectiveNode::Registered(node) => Self::try_from(node),
            TypeDirectiveNode::Link(node) => Ok(Self::Link(node)),
        }
    }
}

impl<'a> TryFrom<&'a CodamaAttribute<'a>> for &'a TypeDirective {
    type Error = CodamaError;

    fn try_from(attribute: &'a CodamaAttribute) -> Result<Self, Self::Error> {
        match attribute.directive.as_ref() {
            CodamaDirective::Type(ref a) => Ok(a),
            _ => Err(CodamaError::InvalidCodamaDirective {
                expected: "type".to_string(),
                actual: attribute.directive.name().to_string(),
            }),
        }
    }
}

impl<'a> TryFrom<&'a Attribute<'a>> for &'a TypeDirective {
    type Error = CodamaError;

    fn try_from(attribute: &'a Attribute) -> Result<Self, Self::Error> {
        <&CodamaAttribute>::try_from(attribute)?.try_into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use codama_nodes::{
        ArrayTypeNode, BooleanTypeNode, ConstantValueNode, DateTimeTypeNode, EnumTypeNode,
        EnumVariantTypeNode, FixedSizeTransformNode, HasTransforms, HiddenPrefixTransformNode,
        HiddenSuffixTransformNode,
        IntegerFormat::{U16, U32, U8},
        IntegerTypeNode, IntegerValueNode, MapTypeNode, OptionTypeNode, PostOffsetTransformNode,
        PreOffsetTransformNode, RemainderOptionTypeNode, SentinelTransformNode, SetTypeNode,
        SizePrefixTransformNode, StringTypeNode, StructFieldTypeNode, StructTypeNode,
        TupleTypeNode, ZeroableOptionTypeNode,
    };
    use syn::parse_quote;

    #[test]
    fn ok() {
        let meta: Meta = parse_quote! { type = integer(u16, le) };
        let directive = TypeDirective::parse(&meta).unwrap();
        assert_eq!(
            directive.node,
            Resolvable::Resolved(RegisteredTypeNode::from(IntegerTypeNode::le(U16)).into())
        );
    }

    #[test]
    fn ok_with_link() {
        let meta: Meta = parse_quote! { type = link("customString") };
        let directive = TypeDirective::parse(&meta).unwrap();
        assert_eq!(
            directive.node,
            Resolvable::Resolved(DefinedTypeLinkNode::new("customString").into())
        );
    }

    #[test]
    fn resolvable_directive() {
        let meta: Meta = parse_quote! { type = foo::custom_type };
        let directive = TypeDirective::parse(&meta).unwrap();

        assert!(directive.node.is_unresolved());
        let Resolvable::Unresolved(ref d) = directive.node else {
            panic!("expected unresolved directive");
        };
        assert_eq!(d.namespace, "foo");
        assert_eq!(d.name, "custom_type");
    }

    #[test]
    fn no_input() {
        let meta: Meta = parse_quote! { type =  };
        let error = TypeDirective::parse(&meta).unwrap_err();
        assert_eq!(error.to_string(), "unrecognized type");
    }

    #[test]
    fn multiple_inputs() {
        let meta: Meta = parse_quote! { type = (integer(u16, le), public_key) };
        let error = TypeDirective::parse(&meta).unwrap_err();
        assert_eq!(error.to_string(), "expected a single value, found a list");
    }

    #[test]
    fn unrecognized_type() {
        let meta: Meta = parse_quote! { type = banana };
        let error = TypeDirective::parse(&meta).unwrap_err();
        assert_eq!(error.to_string(), "unrecognized type");
    }

    #[test]
    fn ok_with_wrapper() {
        let meta: Meta = parse_quote! { type = fixed_size(link("customString"), 8) };
        let directive = TypeDirective::parse(&meta).unwrap();
        assert_eq!(
            directive.node,
            Resolvable::Resolved(
                DefinedTypeLinkNode::new("customString")
                    .add_transform(FixedSizeTransformNode::new(8))
                    .into()
            )
        );
    }

    #[test]
    fn wrappers_in_nested_positions() {
        let string = || StringTypeNode::utf8().add_transform(FixedSizeTransformNode::new(4));
        let integer = || IntegerTypeNode::le(U32).add_transform(FixedSizeTransformNode::new(4));
        let constant = |value: u64| {
            ConstantValueNode::new(IntegerTypeNode::le(U8), IntegerValueNode::new(value))
        };
        for (meta, expected) in [
            (
                parse_quote! { option(fixed_size(string, 4)) },
                TypeNode::from(OptionTypeNode::new(string())),
            ),
            (
                parse_quote! { option(string, prefix = fixed_size(integer(u32), 4)) },
                OptionTypeNode {
                    prefix: integer(),
                    ..OptionTypeNode::new(StringTypeNode::utf8())
                }
                .into(),
            ),
            (
                parse_quote! { zeroable_option(fixed_size(string, 4)) },
                ZeroableOptionTypeNode::new(string()).into(),
            ),
            (
                parse_quote! { remainder_option(fixed_size(string, 4)) },
                RemainderOptionTypeNode::new(string()).into(),
            ),
            (
                parse_quote! { array(fixed_size(string, 4), 3) },
                ArrayTypeNode::fixed(string(), 3).into(),
            ),
            (
                parse_quote! { array(string, prefixed_count(fixed_size(integer(u32), 4))) },
                ArrayTypeNode::prefixed(StringTypeNode::utf8(), integer()).into(),
            ),
            (
                parse_quote! { set(fixed_size(string, 4), 3) },
                SetTypeNode::fixed(string(), 3).into(),
            ),
            (
                parse_quote! { map(fixed_size(string, 4), fixed_size(string, 4), 3) },
                MapTypeNode::fixed(string(), string(), 3).into(),
            ),
            (
                parse_quote! { tuple(fixed_size(string, 4), integer(u32)) },
                TupleTypeNode::new(vec![string().into(), IntegerTypeNode::le(U32).into()]).into(),
            ),
            (
                parse_quote! { struct(field("name", fixed_size(string, 4))) },
                StructTypeNode::new(vec![StructFieldTypeNode::new("name", string())]).into(),
            ),
            (
                parse_quote! { enum(variant("name", fixed_size(string, 4))) },
                EnumTypeNode::new(vec![EnumVariantTypeNode::with_data("name", string())]).into(),
            ),
            (
                parse_quote! { enum(variant("a"), size = fixed_size(integer(u32), 4)) },
                EnumTypeNode {
                    size: integer(),
                    ..EnumTypeNode::new(vec![EnumVariantTypeNode::new("a")])
                }
                .into(),
            ),
            (
                parse_quote! { boolean(fixed_size(integer(u32), 4)) },
                BooleanTypeNode::new(integer()).into(),
            ),
            (
                parse_quote! { size_prefix(string, fixed_size(integer(u32), 4)) },
                StringTypeNode::utf8()
                    .add_transform(SizePrefixTransformNode::new(integer()))
                    .into(),
            ),
            (
                parse_quote! { date_time(fixed_size(integer(u32), 4)) },
                DateTimeTypeNode::new(IntegerTypeNode::le(U32))
                    .add_transform(FixedSizeTransformNode::new(4))
                    .into(),
            ),
            (
                parse_quote! { fixed_size(link("foo"), 4) },
                DefinedTypeLinkNode::new("foo")
                    .add_transform(FixedSizeTransformNode::new(4))
                    .into(),
            ),
            (
                parse_quote! {
                    hidden_suffix(
                        hidden_prefix(
                            sentinel(
                                post_offset(
                                    pre_offset(size_prefix(fixed_size(string, 4), integer(u8)), 1, padded),
                                    2,
                                    absolute
                                ),
                                constant(integer(u8), 0)
                            ),
                            constant(integer(u8), 1)
                        ),
                        constant(integer(u8), 2)
                    )
                },
                string()
                    .add_transform(SizePrefixTransformNode::new(IntegerTypeNode::le(U8)))
                    .add_transform(PreOffsetTransformNode::padded(1))
                    .add_transform(PostOffsetTransformNode::absolute(2))
                    .add_transform(SentinelTransformNode::new(constant(0)))
                    .add_transform(HiddenPrefixTransformNode::new(vec![constant(1)]))
                    .add_transform(HiddenSuffixTransformNode::new(vec![constant(2)]))
                    .into(),
            ),
        ] {
            assert_eq!(TypeNode::from_meta(&meta).unwrap(), expected);
        }
    }
}
