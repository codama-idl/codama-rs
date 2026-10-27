use crate::{
    utils::{FromMeta, SetOnce},
    Attribute, CodamaAttribute, CodamaDirective,
};
use codama_errors::CodamaError;
use codama_nodes::{
    CamelCaseString, DisplaySkip, FieldDiscriminatorNode, IntegerFormat::U8, IntegerTypeNode, Node,
    Path, StructFieldDisplayNode, StructFieldTypeNode, TypeNode, TypeNodeTrait,
};
use codama_syn_helpers::{extensions::*, Meta};

#[derive(Debug, PartialEq, Default, Clone)]
pub struct EnumDiscriminatorDirective {
    pub name: Option<CamelCaseString>,
    pub size: Option<IntegerTypeNode>,
}

impl EnumDiscriminatorDirective {
    pub fn parse(meta: &Meta) -> syn::Result<Self> {
        let pl = meta
            .assert_directive("enum_discriminator")?
            .as_path_list()?;

        let mut name = SetOnce::<CamelCaseString>::new("name");
        let mut size: SetOnce<IntegerTypeNode> = SetOnce::<IntegerTypeNode>::new("size");
        pl.each(|ref meta| match meta.path_str().as_str() {
            "name" => {
                let expr = meta.as_value()?.as_expr()?;
                let value = expr.as_string()?;
                let identifier = CamelCaseString::from(value.as_str());
                // The name is also the path of the discriminator node, so it must be a valid path.
                if Path::try_from(identifier.as_str()).is_err() {
                    return Err(expr.error(format!("Invalid enum discriminator name: {value}")));
                }
                name.set(identifier, meta)
            }
            "size" => {
                let node = TypeNode::from_meta(meta.as_value()?)?;
                match IntegerTypeNode::try_from_type_node(node) {
                    Ok(node) => size.set(node, meta),
                    _ => Err(meta.error("size must be an IntegerTypeNode")),
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

    fn identifier(&self) -> CamelCaseString {
        self.name.clone().unwrap_or("discriminator".into())
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

impl TryFrom<&EnumDiscriminatorDirective> for FieldDiscriminatorNode {
    type Error = CodamaError;

    fn try_from(directive: &EnumDiscriminatorDirective) -> Result<Self, Self::Error> {
        let path = Path::try_from(directive.identifier().as_str()).map_err(|_| {
            CodamaError::InvalidNodeConversion {
                from: "enumDiscriminatorDirective".to_string(),
                into: "fieldDiscriminatorNode".to_string(),
            }
        })?;
        Ok(FieldDiscriminatorNode::new(path, 0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use codama_nodes::IntegerFormat::{U16, U32};

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
        let meta: Meta = syn::parse_quote! { enum_discriminator(size = integer(u32)) };
        let directive = EnumDiscriminatorDirective::parse(&meta).unwrap();
        assert_eq!(
            directive,
            EnumDiscriminatorDirective {
                name: None,
                size: Some(IntegerTypeNode::le(U32)),
            }
        );
    }

    #[test]
    fn enum_discriminator_with_name_and_size() {
        let meta: Meta =
            syn::parse_quote! { enum_discriminator(name = "banana", size = integer(u16)) };
        let directive = EnumDiscriminatorDirective::parse(&meta).unwrap();
        assert_eq!(
            directive,
            EnumDiscriminatorDirective {
                name: Some("banana".into()),
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
            "enum_discriminator must specify at least one of: name, size"
        );
    }

    #[test]
    fn invalid_enum_discriminator_name() {
        let meta: Meta = syn::parse_quote! { enum_discriminator(name = "1st") };
        let error = EnumDiscriminatorDirective::parse(&meta).unwrap_err();
        assert_eq!(error.to_string(), "Invalid enum discriminator name: 1st");
    }

    #[test]
    fn creates_field_discriminator_nodes() {
        for (meta, expected) in [
            (
                syn::parse_quote! { enum_discriminator(size = integer(u16)) },
                "discriminator",
            ),
            (
                syn::parse_quote! { enum_discriminator(name = "my_kind") },
                "myKind",
            ),
        ] {
            let directive = EnumDiscriminatorDirective::parse(&meta).unwrap();
            assert_eq!(
                FieldDiscriminatorNode::try_from(&directive).unwrap(),
                FieldDiscriminatorNode::new(Path::try_from(expected).unwrap(), 0)
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
}
