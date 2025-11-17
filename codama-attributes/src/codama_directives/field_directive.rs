use crate::{
    codama_directives::type_nodes::StructFieldMetaConsumer,
    utils::{FromMeta, MetaConsumer},
    Attribute, CodamaAttribute, CodamaDirective,
};
use codama_errors::CodamaError;
use codama_nodes::{Docs, StructFieldTypeNode};
use codama_syn_helpers::Meta;

#[derive(Debug, PartialEq)]
pub struct FieldDirective {
    pub after: bool,
    pub field: StructFieldTypeNode,
}

impl FieldDirective {
    pub fn parse(meta: &Meta) -> syn::Result<Self> {
        meta.assert_directive("field")?;
        let consumer = StructFieldMetaConsumer::from_meta(meta)?
            .consume_field()?
            .consume_default_value()?
            .consume_after()?
            .assert_fully_consumed()?;

        let default_value = consumer.default_value_node();
        let default_value_strategy = consumer.default_value_strategy();

        Ok(FieldDirective {
            after: consumer.after.option().unwrap_or(false),
            field: StructFieldTypeNode {
                name: consumer.name.take(meta)?,
                r#type: consumer.r#type.take(meta)?,
                docs: Docs::default(),
                default_value,
                default_value_strategy,
            },
        })
    }
}

impl<'a> TryFrom<&'a CodamaAttribute<'a>> for &'a FieldDirective {
    type Error = CodamaError;

    fn try_from(attribute: &'a CodamaAttribute) -> Result<Self, Self::Error> {
        match attribute.directive.as_ref() {
            CodamaDirective::Field(ref a) => Ok(a),
            _ => Err(CodamaError::InvalidCodamaDirective {
                expected: "field".to_string(),
                actual: attribute.directive.name().to_string(),
            }),
        }
    }
}

impl<'a> TryFrom<&'a Attribute<'a>> for &'a FieldDirective {
    type Error = CodamaError;

    fn try_from(attribute: &'a Attribute) -> Result<Self, Self::Error> {
        <&CodamaAttribute>::try_from(attribute)?.try_into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use codama_nodes::{NumberFormat::U8, NumberTypeNode, NumberValueNode};

    #[test]
    fn ok() {
        let meta: Meta = syn::parse_quote! { field("age", number(u8)) };
        let directive = FieldDirective::parse(&meta).unwrap();
        assert_eq!(
            directive,
            FieldDirective {
                after: false,
                field: StructFieldTypeNode::new("age", NumberTypeNode::le(U8)),
            }
        );
    }

    #[test]
    fn after() {
        let meta: Meta = syn::parse_quote! { field(after, "age", number(u8)) };
        let directive = FieldDirective::parse(&meta).unwrap();
        assert_eq!(
            directive,
            FieldDirective {
                after: true,
                field: StructFieldTypeNode::new("age", NumberTypeNode::le(U8)),
            }
        );
    }

    #[test]
    fn with_default_value() {
        let meta: Meta = syn::parse_quote! { field("age", number(u8), default_value = 42) };
        let directive = FieldDirective::parse(&meta).unwrap();
        assert_eq!(
            directive,
            FieldDirective {
                after: false,
                field: StructFieldTypeNode {
                    default_value: Some(NumberValueNode::new(42u8).into()),
                    ..StructFieldTypeNode::new("age", NumberTypeNode::le(U8))
                },
            }
        );
    }
}
