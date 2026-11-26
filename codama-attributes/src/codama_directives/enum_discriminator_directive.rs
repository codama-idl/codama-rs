use crate::{
    utils::{parse_integer_type, removed_name_key, FromMeta, SetOnce},
    Attribute, CodamaAttribute, CodamaDirective,
};
use codama_errors::CodamaError;
use codama_nodes::{
    DisplaySkip, FieldDiscriminatorNode, Identifier, IntegerFormat::U8, IntegerTypeNode, Node,
    StructFieldDisplayNode, StructFieldTypeNode, TypeNode,
};
use codama_syn_helpers::{extensions::*, Meta};

#[derive(Debug, PartialEq, Default, Clone)]
pub struct EnumDiscriminatorDirective {
    pub identifier: Option<Identifier>,
    pub size: Option<IntegerTypeNode>,
}

impl EnumDiscriminatorDirective {
    pub fn parse(meta: &Meta) -> syn::Result<Self> {
        let pl = meta
            .assert_directive("enum_discriminator")?
            .as_path_list()?;

        let mut identifier = SetOnce::<Identifier>::new("identifier");
        let mut size: SetOnce<IntegerTypeNode> = SetOnce::<IntegerTypeNode>::new("size");
        pl.each(|ref meta| match meta.path_str().as_str() {
            "identifier" => identifier.set(Identifier::from_meta(meta)?, meta),
            "name" => removed_name_key(meta),
            "size" => size.set(parse_integer_type(meta.as_value()?, "size")?, meta),
            _ => Err(meta.error("unrecognized attribute")),
        })?;

        let directive = EnumDiscriminatorDirective {
            identifier: identifier.option(),
            size: size.option(),
        };

        if directive.identifier.is_none() && directive.size.is_none() {
            return Err(
                meta.error("enum_discriminator must specify at least one of: identifier, size")
            );
        }

        Ok(directive)
    }

    fn identifier(&self) -> Identifier {
        self.identifier
            .clone()
            .unwrap_or_else(|| Identifier::from("discriminator"))
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
        let size = match node {
            Some(Node::DefinedType(defined_type)) => match defined_type.r#type.as_ref() {
                TypeNode::Enum(data) => Some(data.size.clone()),
                _ => None,
            },
            _ => None,
        };
        EnumDiscriminatorDirective {
            size,
            ..EnumDiscriminatorDirective::default()
        }
    }
}

impl From<&EnumDiscriminatorDirective> for StructFieldTypeNode {
    fn from(directive: &EnumDiscriminatorDirective) -> Self {
        StructFieldTypeNode {
            display: Some(StructFieldDisplayNode::skipped(DisplaySkip::Always)),
            ..StructFieldTypeNode::new(
                directive.identifier(),
                directive.size.clone().unwrap_or(IntegerTypeNode::le(U8)),
            )
        }
    }
}

impl From<&EnumDiscriminatorDirective> for FieldDiscriminatorNode {
    fn from(directive: &EnumDiscriminatorDirective) -> Self {
        FieldDiscriminatorNode::new(directive.identifier(), 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use codama_nodes::{
        Identifier,
        IntegerFormat::{U16, U32},
    };

    #[test]
    fn enum_discriminator_with_name() {
        let meta: Meta = syn::parse_quote! { enum_discriminator(identifier = "banana") };
        let directive = EnumDiscriminatorDirective::parse(&meta).unwrap();
        assert_eq!(
            directive,
            EnumDiscriminatorDirective {
                identifier: Some(Identifier::from("banana")),
                size: None,
            }
        );
    }

    #[test]
    fn enum_discriminator_with_size() {
        let meta: Meta = syn::parse_quote! { enum_discriminator(size = integer(u32)) };
        let directive = EnumDiscriminatorDirective::parse(&meta).unwrap();
        assert_eq!(
            directive,
            EnumDiscriminatorDirective {
                identifier: None,
                size: Some(IntegerTypeNode::le(U32)),
            }
        );
    }

    #[test]
    fn enum_discriminator_with_name_and_size() {
        let meta: Meta =
            syn::parse_quote! { enum_discriminator(identifier = "banana", size = integer(u16)) };
        let directive = EnumDiscriminatorDirective::parse(&meta).unwrap();
        assert_eq!(
            directive,
            EnumDiscriminatorDirective {
                identifier: Some(Identifier::from("banana")),
                size: Some(IntegerTypeNode::le(U16)),
            }
        );
    }

    #[test]
    fn empty_enum_discriminator() {
        let meta: Meta = syn::parse_quote! { enum_discriminator() };
        let error = EnumDiscriminatorDirective::parse(&meta).unwrap_err();
        assert_eq!(
            error.to_string(),
            "enum_discriminator must specify at least one of: identifier, size"
        );
    }

    #[test]
    fn invalid_enum_discriminator_name() {
        let meta: Meta = syn::parse_quote! { enum_discriminator(identifier = "1st") };
        let error = EnumDiscriminatorDirective::parse(&meta).unwrap_err();
        assert_eq!(
            error.to_string(),
            "Invalid identifier `1st`: use letters, digits and underscores, not starting with a digit"
        );
    }

    #[test]
    fn creates_field_discriminator_nodes() {
        for (meta, expected) in [
            (
                syn::parse_quote! { enum_discriminator(size = integer(u16)) },
                "discriminator",
            ),
            (
                syn::parse_quote! { enum_discriminator(identifier = "my_kind") },
                "my_kind",
            ),
        ] {
            let directive = EnumDiscriminatorDirective::parse(&meta).unwrap();
            assert_eq!(
                FieldDiscriminatorNode::from(&directive),
                FieldDiscriminatorNode::new(expected, 0)
            );
        }
    }

    #[test]
    fn creates_hidden_struct_fields() {
        let directive = EnumDiscriminatorDirective::default();
        let field = StructFieldTypeNode::from(&directive);

        assert_eq!(
            field.display,
            Some(StructFieldDisplayNode::skipped(DisplaySkip::Always))
        );
    }

    #[test]
    fn name_is_removed() {
        let meta: Meta = syn::parse_quote! { enum_discriminator(name = "kind") };
        let error = EnumDiscriminatorDirective::parse(&meta).unwrap_err();
        assert_eq!(
            error.to_string(),
            "name = ... is removed in v2, use identifier = ..."
        );
    }
}
