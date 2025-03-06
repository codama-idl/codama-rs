use crate::{utils::FromMeta, Attribute, CodamaAttribute, CodamaDirective};
use codama_errors::CodamaError;
use codama_nodes::{NestedTypeNode, NumberTypeNode, TypeNode};
use codama_syn_helpers::{extensions::*, Meta};

#[derive(Debug, PartialEq)]
pub struct SizePrefixDirective {
    pub prefix: NestedTypeNode<NumberTypeNode>,
}

impl SizePrefixDirective {
    pub fn parse(meta: &Meta) -> syn::Result<Self> {
        let pv = meta.assert_directive("size_prefix")?.as_path_value()?;
        let node = TypeNode::from_meta(&pv.value)?;
        let prefix = match NestedTypeNode::<NumberTypeNode>::try_from(node) {
            Ok(node) => node,
            Err(_) => return Err(pv.value.error("prefix must be a NumberTypeNode")),
        };
        Ok(Self { prefix })
    }
}

impl<'a> TryFrom<&'a CodamaAttribute<'a>> for &'a SizePrefixDirective {
    type Error = CodamaError;

    fn try_from(attribute: &'a CodamaAttribute) -> Result<Self, Self::Error> {
        match attribute.directive {
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
    use codama_nodes::NumberFormat::U32;

    #[test]
    fn ok() {
        let meta: Meta = syn::parse_quote! { size_prefix = number(u32) };
        let directive = SizePrefixDirective::parse(&meta).unwrap();
        assert_eq!(
            directive,
            SizePrefixDirective {
                prefix: NumberTypeNode::le(U32).into(),
            }
        );
    }
}
