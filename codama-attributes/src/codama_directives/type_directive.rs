use crate::{utils::FromMeta, Attribute, CodamaAttribute, CodamaDirective};
use codama_errors::CodamaError;
use codama_nodes::RegisteredTypeNode;
use codama_syn_helpers::Meta;

#[derive(Debug, PartialEq)]
pub struct TypeDirective {
    pub node: RegisteredTypeNode,
}

impl TypeDirective {
    pub fn parse(meta: &Meta) -> syn::Result<Self> {
        let pv = meta.assert_directive("type")?.as_path_value()?;
        let node = RegisteredTypeNode::from_meta(&pv.value)?;
        Ok(Self { node })
    }
}

impl<'a> TryFrom<&'a CodamaAttribute<'a>> for &'a TypeDirective {
    type Error = CodamaError;

    fn try_from(attribute: &'a CodamaAttribute) -> Result<Self, Self::Error> {
        match attribute.directive {
            CodamaDirective::Type(ref a) => Ok(a),
            _ => Err(CodamaError::InvalidCodamaDirective {
                expected: "type".to_string(),
                actual: attribute.directive.name().to_string(),
            }),
        }
    }
}

impl<'a> TryFrom<&'a Attribute<'a>> for &'a TypeDirective {
    type Error = CodamaError;

    fn try_from(attribute: &'a Attribute) -> Result<Self, Self::Error> {
        <&CodamaAttribute>::try_from(attribute)?.try_into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use codama_nodes::{NumberFormat::U16, NumberTypeNode};
    use syn::parse_quote;

    #[test]
    fn ok() {
        let meta: Meta = parse_quote! { type = number(u16, le) };
        let node = TypeDirective::parse(&meta).unwrap().node;

        assert_eq!(node, NumberTypeNode::le(U16).into());
    }

    #[test]
    fn no_input() {
        let meta: Meta = parse_quote! { type =  };
        let error = TypeDirective::parse(&meta).unwrap_err();
        assert_eq!(error.to_string(), "unrecognized type");
    }

    #[test]
    fn multiple_inputs() {
        let meta: Meta = parse_quote! { type = (number(u16, le), public_key) };
        let error = TypeDirective::parse(&meta).unwrap_err();
        assert_eq!(error.to_string(), "expected a single value, found a list");
    }

    #[test]
    fn unrecognized_type() {
        let meta: Meta = parse_quote! { type = banana };
        let error = TypeDirective::parse(&meta).unwrap_err();
        assert_eq!(error.to_string(), "unrecognized type");
    }
}
