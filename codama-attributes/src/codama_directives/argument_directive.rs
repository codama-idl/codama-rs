use crate::{
    codama_directives::type_nodes::StructFieldMetaConsumer,
    utils::{FromMeta, MetaConsumer},
    Attribute, CodamaAttribute, CodamaDirective, Resolvable,
};
use codama_errors::{CodamaError, CodamaResult};
use codama_nodes::{
    CamelCaseString, DefaultValueStrategy, Docs, InstructionArgumentNode,
    InstructionInputValueNode, TypeNode,
};
use codama_syn_helpers::Meta;

#[derive(Debug, PartialEq)]
pub struct ArgumentDirective {
    pub after: bool,
    pub name: CamelCaseString,
    pub r#type: Resolvable<TypeNode>,
    pub docs: Docs,
    pub default_value: Option<Resolvable<InstructionInputValueNode>>,
    pub default_value_strategy: Option<DefaultValueStrategy>,
}

impl ArgumentDirective {
    pub fn parse(meta: &Meta) -> syn::Result<Self> {
        meta.assert_directive("argument")?;
        let consumer = StructFieldMetaConsumer::from_meta(meta)?
            .consume_field()?
            .consume_argument_default_value()?
            .consume_after()?
            .assert_fully_consumed()?;

        let default_value = consumer.default_instruction_input_value_node();
        let default_value_strategy = consumer.default_value_strategy();

        Ok(ArgumentDirective {
            after: consumer.after.option().unwrap_or(false),
            name: consumer.name.take(meta)?,
            r#type: consumer.r#type.take(meta)?,
            docs: consumer.docs.option().unwrap_or_default(),
            default_value,
            default_value_strategy,
        })
    }

    /// Construct an `InstructionArgumentNode` from this directive.
    /// Returns an error if any unresolved directives remain.
    pub fn to_instruction_argument_node(&self) -> CodamaResult<InstructionArgumentNode> {
        Ok(InstructionArgumentNode {
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

impl<'a> TryFrom<&'a CodamaAttribute<'a>> for &'a ArgumentDirective {
    type Error = CodamaError;

    fn try_from(attribute: &'a CodamaAttribute) -> Result<Self, Self::Error> {
        match attribute.directive.as_ref() {
            CodamaDirective::Argument(ref a) => Ok(a),
            _ => Err(CodamaError::InvalidCodamaDirective {
                expected: "argument".to_string(),
                actual: attribute.directive.name().to_string(),
            }),
        }
    }
}

impl<'a> TryFrom<&'a Attribute<'a>> for &'a ArgumentDirective {
    type Error = CodamaError;

    fn try_from(attribute: &'a Attribute) -> Result<Self, Self::Error> {
        <&CodamaAttribute>::try_from(attribute)?.try_into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use codama_nodes::{NumberFormat::U8, NumberTypeNode, PayerValueNode};

    #[test]
    fn ok() {
        let meta: Meta = syn::parse_quote! { argument("age", number(u8)) };
        let directive = ArgumentDirective::parse(&meta).unwrap();
        assert_eq!(
            directive,
            ArgumentDirective {
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
        let meta: Meta = syn::parse_quote! { argument(after, "age", number(u8)) };
        let directive = ArgumentDirective::parse(&meta).unwrap();
        assert_eq!(
            directive,
            ArgumentDirective {
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
        let meta: Meta = syn::parse_quote! { argument("age", number(u8), default_value = payer) };
        let directive = ArgumentDirective::parse(&meta).unwrap();
        assert_eq!(
            directive,
            ArgumentDirective {
                after: false,
                name: "age".into(),
                r#type: Resolvable::Resolved(NumberTypeNode::le(U8).into()),
                docs: Docs::default(),
                default_value: Some(Resolvable::Resolved(PayerValueNode::new().into())),
                default_value_strategy: None,
            }
        );
    }

    #[test]
    fn with_docs_string() {
        let meta: Meta = syn::parse_quote! { argument("cake", number(u8), docs = "The cake") };
        let directive = ArgumentDirective::parse(&meta).unwrap();
        assert_eq!(directive.docs, vec!["The cake".to_string()].into());
    }

    #[test]
    fn with_docs_array() {
        let meta: Meta = syn::parse_quote! { argument("cake", number(u8), docs = ["The cake", "must be a lie"]) };
        let directive = ArgumentDirective::parse(&meta).unwrap();
        assert_eq!(
            directive.docs,
            vec!["The cake".to_string(), "must be a lie".to_string()].into()
        );
    }
}
