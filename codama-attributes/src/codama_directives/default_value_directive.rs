use crate::{utils::FromMeta, Attribute, CodamaAttribute, CodamaDirective};
use codama_errors::CodamaError;
use codama_nodes::InstructionInputValueNode;
use codama_syn_helpers::Meta;

#[derive(Debug, PartialEq)]
pub struct DefaultValueDirective {
    pub node: InstructionInputValueNode,
}

impl DefaultValueDirective {
    pub fn parse(meta: &Meta) -> syn::Result<Self> {
        let pv = meta.assert_directive("default_value")?.as_path_value()?;
        let node = InstructionInputValueNode::from_meta(&pv.value)?;
        Ok(Self { node })
    }
}

impl<'a> TryFrom<&'a CodamaAttribute<'a>> for &'a DefaultValueDirective {
    type Error = CodamaError;

    fn try_from(attribute: &'a CodamaAttribute) -> Result<Self, Self::Error> {
        match attribute.directive {
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
    use codama_nodes::BooleanValueNode;
    use syn::parse_quote;

    #[test]
    fn ok() {
        let meta: Meta = parse_quote! { default_value = true };
        let node = DefaultValueDirective::parse(&meta).unwrap().node;

        assert_eq!(node, BooleanValueNode::new(true).into());
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
