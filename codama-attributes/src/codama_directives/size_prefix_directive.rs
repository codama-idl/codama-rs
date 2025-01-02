use crate::utils::FromMeta;
use codama_nodes::{NestedTypeNode, NumberTypeNode, TypeNode};
use codama_syn_helpers::{extensions::*, Meta};

#[derive(Debug, PartialEq)]
pub struct SizePrefixDirective {
    pub prefix: NestedTypeNode<NumberTypeNode>,
}

impl TryFrom<&Meta> for SizePrefixDirective {
    type Error = syn::Error;

    fn try_from(meta: &Meta) -> syn::Result<Self> {
        let pv = meta.assert_directive("size_prefix")?.as_path_value()?;
        let node = TypeNode::from_meta(&pv.value)?;
        let prefix = match NestedTypeNode::<NumberTypeNode>::try_from(node) {
            Ok(node) => node,
            Err(_) => return Err(pv.value.error("prefix must be a NumberTypeNode")),
        };
        Ok(Self { prefix })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use codama_nodes::NumberFormat::U32;

    #[test]
    fn ok() {
        let meta: Meta = syn::parse_quote! { size_prefix = number(u32) };
        let directive = SizePrefixDirective::try_from(&meta).unwrap();
        assert_eq!(
            directive,
            SizePrefixDirective {
                prefix: NumberTypeNode::le(U32).into(),
            }
        );
    }
}