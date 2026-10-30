use crate::{
    codama_directives::type_nodes::StructFieldMetaConsumer,
    utils::{FromMeta, MetaConsumer},
    Attribute, CodamaAttribute, CodamaDirective, DefaultValueDirective, Resolvable,
};
use codama_errors::{CodamaError, CodamaResult};
use codama_nodes::{
    CamelCaseString, DefaultValueStrategy, Docs, InstructionInputValueNode, StructFieldTypeNode,
    TypeNode, ValueNode,
};
use codama_syn_helpers::Meta;

#[derive(Debug, PartialEq)]
pub struct FieldDirective {
    pub after: bool,
    pub name: CamelCaseString,
    pub r#type: Resolvable<TypeNode>,
    pub docs: Docs,
    /// A value, or a contextual value such as `payer` that only an instruction can provide.
    pub default_value: Option<Resolvable<InstructionInputValueNode>>,
    pub default_value_strategy: Option<DefaultValueStrategy>,
}

impl FieldDirective {
    pub fn parse(meta: &Meta) -> syn::Result<Self> {
        meta.assert_directive("field")?;
        let consumer = StructFieldMetaConsumer::from_meta(meta)?
            // `after` goes first, because `consume_field` reads every other bare path as a type.
            .consume_after()?
            .consume_field()?
            .consume_default_value(DefaultValueDirective::parse)?
            .assert_fully_consumed()?;

        let default_value_strategy = consumer.default_value_strategy();
        let default_value = consumer
            .default_value
            .option()
            .map(|directive| directive.node);

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
    /// A contextual default value is not a value node, so the field keeps no default for it.
    /// Returns an error if any unresolved directives remain.
    pub fn to_struct_field_type_node(&self) -> CodamaResult<StructFieldTypeNode> {
        let r#type = self.r#type.try_resolved()?.clone();
        let default_value = self
            .default_value
            .as_ref()
            .map(|node| node.try_resolved())
            .transpose()?
            .and_then(|node| ValueNode::try_from(node.clone()).ok());
        Ok(StructFieldTypeNode {
            identifier: self.name.clone(),
            r#type: Box::new(r#type),
            docs: self.docs.clone(),
            default_value_strategy: default_value.as_ref().and(self.default_value_strategy),
            default_value: Box::new(default_value),
            display: None,
            plugins: vec![],
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
    use codama_nodes::{
        FixedSizeTransformNode, HasTransforms, IntegerFormat::U8, IntegerTypeNode,
        IntegerValueNode, PayerValueNode, PublicKeyTypeNode, StringTypeNode,
    };

    #[test]
    fn ok() {
        let meta: Meta = syn::parse_quote! { field("age", integer(u8)) };
        let directive = FieldDirective::parse(&meta).unwrap();
        assert_eq!(
            directive,
            FieldDirective {
                after: false,
                name: "age".into(),
                r#type: Resolvable::Resolved(IntegerTypeNode::le(U8).into()),
                docs: Docs::default(),
                default_value: None,
                default_value_strategy: None,
            }
        );
    }

    #[test]
    fn after() {
        let meta: Meta = syn::parse_quote! { field(after, "age", integer(u8)) };
        let directive = FieldDirective::parse(&meta).unwrap();
        assert_eq!(
            directive,
            FieldDirective {
                after: true,
                name: "age".into(),
                r#type: Resolvable::Resolved(IntegerTypeNode::le(U8).into()),
                docs: Docs::default(),
                default_value: None,
                default_value_strategy: None,
            }
        );
    }

    #[test]
    fn wrapped_type() {
        let meta: Meta = syn::parse_quote! { field("name", fixed_size(string, 4)) };
        let directive = FieldDirective::parse(&meta).unwrap();
        assert_eq!(
            directive,
            FieldDirective {
                after: false,
                name: "name".into(),
                r#type: Resolvable::Resolved(
                    StringTypeNode::utf8()
                        .add_transform(FixedSizeTransformNode::new(4))
                        .into()
                ),
                docs: Docs::default(),
                default_value: None,
                default_value_strategy: None,
            }
        );
    }

    #[test]
    fn with_default_value() {
        let meta: Meta = syn::parse_quote! { field("age", integer(u8), default_value = 42) };
        let directive = FieldDirective::parse(&meta).unwrap();
        assert_eq!(
            directive,
            FieldDirective {
                after: false,
                name: "age".into(),
                r#type: Resolvable::Resolved(IntegerTypeNode::le(U8).into()),
                docs: Docs::default(),
                default_value: Some(Resolvable::Resolved(IntegerValueNode::new(42u8).into())),
                default_value_strategy: None,
            }
        );
    }

    #[test]
    fn with_contextual_default_value() {
        for (meta, default_value_strategy) in [
            (
                syn::parse_quote! { field("owner", public_key, default_value = payer) },
                None,
            ),
            (
                syn::parse_quote! { field("owner", public_key, value = payer) },
                Some(DefaultValueStrategy::Omitted),
            ),
        ] {
            let directive = FieldDirective::parse(&meta).unwrap();
            assert_eq!(
                directive,
                FieldDirective {
                    after: false,
                    name: "owner".into(),
                    r#type: Resolvable::Resolved(PublicKeyTypeNode::new().into()),
                    docs: Docs::default(),
                    default_value: Some(Resolvable::Resolved(PayerValueNode::new().into())),
                    default_value_strategy,
                }
            );
        }
    }

    #[test]
    fn struct_field_type_node_keeps_value_defaults_only() {
        for (meta, expected) in [
            (
                syn::parse_quote! { field("age", integer(u8), value = 42) },
                StructFieldTypeNode {
                    default_value: Box::new(Some(IntegerValueNode::new(42u8).into())),
                    default_value_strategy: Some(DefaultValueStrategy::Omitted),
                    ..StructFieldTypeNode::new("age", IntegerTypeNode::le(U8))
                },
            ),
            (
                syn::parse_quote! { field("owner", public_key, value = payer) },
                StructFieldTypeNode::new("owner", PublicKeyTypeNode::new()),
            ),
        ] {
            let directive = FieldDirective::parse(&meta).unwrap();
            assert_eq!(directive.to_struct_field_type_node().unwrap(), expected);
        }
    }

    #[test]
    fn invalid_type() {
        let meta: Meta = syn::parse_quote! { field("len", number(u8), default_value = 5) };
        let error = FieldDirective::parse(&meta).unwrap_err();
        assert_eq!(
            error.to_string(),
            "number(...) is removed in v2, use integer(...) or float(...)"
        );
    }

    #[test]
    fn with_docs_string() {
        let meta: Meta = syn::parse_quote! { field("splines", integer(u8), docs = "Splines") };
        let directive = FieldDirective::parse(&meta).unwrap();
        assert_eq!(directive.docs, vec!["Splines".to_string()].into());
    }

    #[test]
    fn with_docs_array() {
        let meta: Meta = syn::parse_quote! { field("age", integer(u8), docs = ["Splines", "Must be pre-reticulated"]) };
        let directive = FieldDirective::parse(&meta).unwrap();
        assert_eq!(
            directive.docs,
            vec!["Splines".to_string(), "Must be pre-reticulated".to_string()].into()
        );
    }
}
