use crate::{
    codama_directives::type_nodes::StructFieldMetaConsumer,
    utils::{FromMeta, MetaConsumer},
    Attribute, CodamaAttribute, CodamaDirective,
};
use codama_errors::CodamaError;
use codama_nodes::{Docs, InstructionArgumentNode};
use codama_syn_helpers::Meta;

#[derive(Debug, PartialEq)]
pub struct ArgumentDirective {
    pub after: bool,
    pub argument: InstructionArgumentNode,
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
            argument: InstructionArgumentNode {
                name: consumer.name.take(meta)?,
                r#type: consumer.r#type.take(meta)?,
                docs: Docs::default(),
                default_value,
                default_value_strategy,
            },
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
                argument: InstructionArgumentNode::new("age", NumberTypeNode::le(U8)),
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
                argument: InstructionArgumentNode::new("age", NumberTypeNode::le(U8)),
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
                argument: InstructionArgumentNode {
                    default_value: Some(PayerValueNode::new().into()),
                    ..InstructionArgumentNode::new("age", NumberTypeNode::le(U8))
                },
            }
        );
    }
}
