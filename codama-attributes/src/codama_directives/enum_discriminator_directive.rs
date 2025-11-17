use crate::{
    utils::{FromMeta, SetOnce},
    Attribute, CodamaAttribute, CodamaDirective,
};
use codama_errors::CodamaError;
use codama_nodes::{
    CamelCaseString, DefinedTypeNode, InstructionArgumentNode, NestedTypeNode, Node,
    NumberFormat::U8, NumberTypeNode, StructFieldTypeNode, TypeNode,
};
use codama_syn_helpers::{extensions::*, Meta};

#[derive(Debug, PartialEq, Default, Clone)]
pub struct EnumDiscriminatorDirective {
    pub name: Option<CamelCaseString>,
    pub size: Option<NestedTypeNode<NumberTypeNode>>,
}

impl EnumDiscriminatorDirective {
    pub fn parse(meta: &Meta) -> syn::Result<Self> {
        let pl = meta
            .assert_directive("enum_discriminator")?
            .as_path_list()?;

        let mut name = SetOnce::<CamelCaseString>::new("name");
        let mut size: SetOnce<NestedTypeNode<NumberTypeNode>> =
            SetOnce::<NestedTypeNode<NumberTypeNode>>::new("size");
        pl.each(|ref meta| match meta.path_str().as_str() {
            "name" => name.set(meta.as_value()?.as_expr()?.as_string()?.into(), meta),
            "size" => {
                let node = TypeNode::from_meta(meta.as_value()?)?;
                match NestedTypeNode::<NumberTypeNode>::try_from(node) {
                    Ok(node) => size.set(node, meta),
                    _ => Err(meta.error("size must be a NumberTypeNode")),
                }
            }
            _ => Err(meta.error("unrecognized attribute")),
        })?;

        let directive = EnumDiscriminatorDirective {
            name: name.option(),
            size: size.option(),
        };

        if directive.name.is_none() && directive.size.is_none() {
            return Err(meta.error("enum_discriminator must specify at least one of: name, size"));
        }

        Ok(directive)
    }
}

impl<'a> TryFrom<&'a CodamaAttribute<'a>> for &'a EnumDiscriminatorDirective {
    type Error = CodamaError;

    fn try_from(attribute: &'a CodamaAttribute) -> Result<Self, Self::Error> {
        match attribute.directive.as_ref() {
            CodamaDirective::EnumDiscriminator(ref a) => Ok(a),
            _ => Err(CodamaError::InvalidCodamaDirective {
                expected: "enum_discriminator".to_string(),
                actual: attribute.directive.name().to_string(),
            }),
        }
    }
}

impl<'a> TryFrom<&'a Attribute<'a>> for &'a EnumDiscriminatorDirective {
    type Error = CodamaError;

    fn try_from(attribute: &'a Attribute) -> Result<Self, Self::Error> {
        <&CodamaAttribute>::try_from(attribute)?.try_into()
    }
}

impl From<&Option<Node>> for EnumDiscriminatorDirective {
    fn from(node: &Option<Node>) -> Self {
        EnumDiscriminatorDirective {
            size: match &node {
                Some(Node::DefinedType(DefinedTypeNode {
                    r#type: TypeNode::Enum(data),
                    ..
                })) => Some(data.size.clone()),
                _ => None,
            },
            ..EnumDiscriminatorDirective::default()
        }
    }
}

impl From<&EnumDiscriminatorDirective> for StructFieldTypeNode {
    fn from(directive: &EnumDiscriminatorDirective) -> Self {
        StructFieldTypeNode::new(
            directive.name.clone().unwrap_or("discriminator".into()),
            directive
                .size
                .clone()
                .unwrap_or(NumberTypeNode::le(U8).into()),
        )
    }
}

impl From<&EnumDiscriminatorDirective> for InstructionArgumentNode {
    fn from(directive: &EnumDiscriminatorDirective) -> Self {
        StructFieldTypeNode::from(directive).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use codama_nodes::NumberFormat::{U16, U32};

    #[test]
    fn enum_discriminator_with_name() {
        let meta: Meta = syn::parse_quote! { enum_discriminator(name = "banana") };
        let directive = EnumDiscriminatorDirective::parse(&meta).unwrap();
        assert_eq!(
            directive,
            EnumDiscriminatorDirective {
                name: Some("banana".into()),
                size: None,
            }
        );
    }

    #[test]
    fn enum_discriminator_with_size() {
        let meta: Meta = syn::parse_quote! { enum_discriminator(size = number(u32)) };
        let directive = EnumDiscriminatorDirective::parse(&meta).unwrap();
        assert_eq!(
            directive,
            EnumDiscriminatorDirective {
                name: None,
                size: Some(NumberTypeNode::le(U32).into()),
            }
        );
    }

    #[test]
    fn enum_discriminator_with_name_and_size() {
        let meta: Meta =
            syn::parse_quote! { enum_discriminator(name = "banana", size = number(u16)) };
        let directive = EnumDiscriminatorDirective::parse(&meta).unwrap();
        assert_eq!(
            directive,
            EnumDiscriminatorDirective {
                name: Some("banana".into()),
                size: Some(NumberTypeNode::le(U16).into()),
            }
        );
    }

    #[test]
    fn empty_enum_discriminator() {
        let meta: Meta = syn::parse_quote! { enum_discriminator() };
        let error = EnumDiscriminatorDirective::parse(&meta).unwrap_err();
        assert_eq!(
            error.to_string(),
            "enum_discriminator must specify at least one of: name, size"
        );
    }
}
