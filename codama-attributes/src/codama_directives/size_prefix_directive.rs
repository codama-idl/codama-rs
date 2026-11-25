use crate::{utils::parse_integer_type, Attribute, CodamaAttribute, CodamaDirective};
use codama_errors::CodamaError;
use codama_nodes::IntegerTypeNode;
use codama_syn_helpers::Meta;

#[derive(Debug, PartialEq)]
pub struct SizePrefixDirective {
    pub prefix: IntegerTypeNode,
}

impl SizePrefixDirective {
    pub fn parse(meta: &Meta) -> syn::Result<Self> {
        let value = meta.assert_directive("size_prefix")?.as_value()?;
        let prefix = parse_integer_type(value, "prefix")?;
        Ok(Self { prefix })
    }
}

impl<'a> TryFrom<&'a CodamaAttribute<'a>> for &'a SizePrefixDirective {
    type Error = CodamaError;

    fn try_from(attribute: &'a CodamaAttribute) -> Result<Self, Self::Error> {
        match attribute.directive.as_ref() {
            CodamaDirective::SizePrefix(ref a) => Ok(a),
            _ => Err(CodamaError::InvalidCodamaDirective {
                expected: "size_prefix".to_string(),
                actual: attribute.directive.name().to_string(),
            }),
        }
    }
}

impl<'a> TryFrom<&'a Attribute<'a>> for &'a SizePrefixDirective {
    type Error = CodamaError;

    fn try_from(attribute: &'a Attribute) -> Result<Self, Self::Error> {
        <&CodamaAttribute>::try_from(attribute)?.try_into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use codama_nodes::IntegerFormat::U32;

    #[test]
    fn ok() {
        let meta: Meta = syn::parse_quote! { size_prefix = integer(u32) };
        let directive = SizePrefixDirective::parse(&meta).unwrap();
        assert_eq!(
            directive,
            SizePrefixDirective {
                prefix: IntegerTypeNode::le(U32),
            }
        );
    }
}
