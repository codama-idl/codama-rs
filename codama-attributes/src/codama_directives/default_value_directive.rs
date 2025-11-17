use crate::{utils::FromMeta, Attribute, CodamaAttribute, CodamaDirective};
use codama_errors::CodamaError;
use codama_nodes::{DefaultValueStrategy, InstructionInputValueNode, ValueNode};
use codama_syn_helpers::{extensions::*, Meta};

#[derive(Debug, PartialEq)]
pub struct DefaultValueDirective {
    pub node: InstructionInputValueNode,
    pub default_value_strategy: Option<DefaultValueStrategy>,
}

impl DefaultValueDirective {
    pub fn parse(meta: &Meta) -> syn::Result<Self> {
        Self::parse_logic(meta, false)
    }

    pub fn parse_value_nodes_only(meta: &Meta) -> syn::Result<Self> {
        Self::parse_logic(meta, true)
    }

    fn parse_logic(meta: &Meta, value_nodes_only: bool) -> syn::Result<Self> {
        let pv = meta.as_path_value()?;
        let is_default_value = pv.path.is_strict("default_value");
        let is_value = pv.path.is_strict("value");
        if !is_default_value && !is_value {
            return Err(pv
                .path
                .error("expected #[codama(default_value)] or #[codama(value)] attributes"));
        };

        let node = match value_nodes_only {
            true => ValueNode::from_meta(&pv.value)?.into(),
            false => InstructionInputValueNode::from_meta(&pv.value)?,
        };

        let default_value_strategy = match is_value {
            true => Some(DefaultValueStrategy::Omitted),
            false => None,
        };

        Ok(Self {
            node,
            default_value_strategy,
        })
    }
}

impl<'a> TryFrom<&'a CodamaAttribute<'a>> for &'a DefaultValueDirective {
    type Error = CodamaError;

    fn try_from(attribute: &'a CodamaAttribute) -> Result<Self, Self::Error> {
        match attribute.directive.as_ref() {
            CodamaDirective::DefaultValue(ref a) => Ok(a),
            _ => Err(CodamaError::InvalidCodamaDirective {
                expected: "default_value".to_string(),
                actual: attribute.directive.name().to_string(),
            }),
        }
    }
}

impl<'a> TryFrom<&'a Attribute<'a>> for &'a DefaultValueDirective {
    type Error = CodamaError;

    fn try_from(attribute: &'a Attribute) -> Result<Self, Self::Error> {
        <&CodamaAttribute>::try_from(attribute)?.try_into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use codama_nodes::{BooleanValueNode, PayerValueNode};
    use syn::parse_quote;

    #[test]
    fn default_value_ok() {
        let meta: Meta = parse_quote! { default_value = payer };
        let directive = DefaultValueDirective::parse(&meta).unwrap();

        assert_eq!(
            directive,
            DefaultValueDirective {
                node: PayerValueNode::new().into(),
                default_value_strategy: None,
            }
        );
    }

    #[test]
    fn value_ok() {
        let meta: Meta = parse_quote! { value = payer };
        let directive = DefaultValueDirective::parse(&meta).unwrap();

        assert_eq!(
            directive,
            DefaultValueDirective {
                node: PayerValueNode::new().into(),
                default_value_strategy: Some(DefaultValueStrategy::Omitted),
            }
        );
    }

    #[test]
    fn parse_value_nodes_only_ok() {
        let meta: Meta = parse_quote! { value = true };
        let directive = DefaultValueDirective::parse_value_nodes_only(&meta).unwrap();

        assert_eq!(
            directive,
            DefaultValueDirective {
                node: BooleanValueNode::new(true).into(),
                default_value_strategy: Some(DefaultValueStrategy::Omitted),
            }
        );
    }

    #[test]
    fn parse_value_nodes_only_err() {
        let meta: Meta = parse_quote! { value = payer };
        let error = DefaultValueDirective::parse_value_nodes_only(&meta).unwrap_err();
        assert_eq!(error.to_string(), "unrecognized value");
    }

    #[test]
    fn no_input() {
        let meta: Meta = parse_quote! { default_value =  };
        let error = DefaultValueDirective::parse(&meta).unwrap_err();
        assert_eq!(error.to_string(), "unrecognized value");
    }

    #[test]
    fn multiple_inputs() {
        let meta: Meta = parse_quote! { default_value = (true, false) };
        let error = DefaultValueDirective::parse(&meta).unwrap_err();
        assert_eq!(error.to_string(), "expected a single value, found a list");
    }

    #[test]
    fn unrecognized_value() {
        let meta: Meta = parse_quote! { default_value = banana };
        let error = DefaultValueDirective::parse(&meta).unwrap_err();
        assert_eq!(error.to_string(), "unrecognized value");
    }
}
