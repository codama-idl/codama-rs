use crate::{
    codama_directives::type_nodes::StructFieldMetaConsumer,
    utils::{FromMeta, MetaConsumer},
    Attribute, CodamaAttribute, CodamaDirective, Resolvable,
};
use codama_errors::{CodamaError, CodamaResult};
use codama_nodes::{
    CamelCaseString, DefaultValueStrategy, Docs, StructFieldTypeNode, TypeNode, ValueNode,
};
use codama_syn_helpers::Meta;

#[derive(Debug, PartialEq)]
pub struct FieldDirective {
    pub after: bool,
    pub name: CamelCaseString,
    pub r#type: Resolvable<TypeNode>,
    pub docs: Docs,
    pub default_value: Option<Resolvable<ValueNode>>,
    pub default_value_strategy: Option<DefaultValueStrategy>,
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
            name: consumer.name.take(meta)?,
            r#type: consumer.r#type.take(meta)?,
            docs: consumer.docs.option().unwrap_or_default(),
            default_value,
            default_value_strategy,
        })
    }

    /// Construct a `StructFieldTypeNode` from this directive.
    /// Returns an error if any unresolved directives remain.
    pub fn to_struct_field_type_node(&self) -> CodamaResult<StructFieldTypeNode> {
        Ok(StructFieldTypeNode {
            name: self.name.clone(),
            r#type: self.r#type.try_resolved()?.clone(),
            docs: self.docs.clone(),
            default_value: self
                .default_value
                .as_ref()
                .map(|r| r.try_resolved().cloned())
                .transpose()?,
            default_value_strategy: self.default_value_strategy,
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
                name: "age".into(),
                r#type: Resolvable::Resolved(NumberTypeNode::le(U8).into()),
                docs: Docs::default(),
                default_value: None,
                default_value_strategy: None,
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
                name: "age".into(),
                r#type: Resolvable::Resolved(NumberTypeNode::le(U8).into()),
                docs: Docs::default(),
                default_value: None,
                default_value_strategy: None,
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
                name: "age".into(),
                r#type: Resolvable::Resolved(NumberTypeNode::le(U8).into()),
                docs: Docs::default(),
                default_value: Some(Resolvable::Resolved(NumberValueNode::new(42u8).into())),
                default_value_strategy: None,
            }
        );
    }

    #[test]
    fn with_docs_string() {
        let meta: Meta = syn::parse_quote! { field("splines", number(u8), docs = "Splines") };
        let directive = FieldDirective::parse(&meta).unwrap();
        assert_eq!(directive.docs, vec!["Splines".to_string()].into());
    }

    #[test]
    fn with_docs_array() {
        let meta: Meta = syn::parse_quote! { field("age", number(u8), docs = ["Splines", "Must be pre-reticulated"]) };
        let directive = FieldDirective::parse(&meta).unwrap();
        assert_eq!(
            directive.docs,
            vec!["Splines".to_string(), "Must be pre-reticulated".to_string()].into()
        );
    }
}
