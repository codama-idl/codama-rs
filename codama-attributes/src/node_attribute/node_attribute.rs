use crate::NodeAttributeParse;
use codama_errors::{CodamaError, CodamaResult};
use codama_nodes::Node;
use codama_syn_helpers::syn_traits::*;

#[derive(Debug, PartialEq)]
pub struct NodeAttribute<'a> {
    pub ast: &'a syn::Attribute,
    pub node: Node,
}

impl<'a> TryFrom<&'a syn::Attribute> for NodeAttribute<'a> {
    type Error = CodamaError;

    fn try_from(ast: &'a syn::Attribute) -> CodamaResult<Self> {
        // Check if the attribute is feature-gated.
        let unfeatured = ast.unfeatured();
        let attr = unfeatured.as_ref().unwrap_or(ast);

        // Check if the attribute is a #[node(...)] attribute.
        let list = attr.meta.require_list()?;
        if !list.path.is_strict("node") {
            return Err(syn::Error::new_spanned(&list.path, "expected #[node(...)]").into());
        };

        // Parse the node from the token stream.
        let mut node: CodamaResult<Node> =
            Err(syn::Error::new_spanned(&list.tokens, "empty node").into());
        attr.parse_nested_meta(|meta| {
            node = Node::from_meta(&meta);
            Ok(())
        })?;
        Ok(Self { ast, node: node? })
    }
}

impl<'a> NodeAttribute<'a> {
    pub fn parse<T: TryInto<Self, Error = CodamaError>>(attr: T) -> CodamaResult<Self> {
        attr.try_into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use codama_nodes::{NumberFormat::U16, NumberTypeNode};
    use codama_syn_helpers::syn_build;
    use quote::quote;

    #[test]
    fn test_type_attribute() {
        let ast = syn_build::attribute(quote! { #[node(numberTypeNode(u16, le))] });
        let attribute = NodeAttribute::parse(&ast).unwrap();

        assert_eq!(attribute.ast, &ast);
        assert_eq!(attribute.node, NumberTypeNode::le(U16).into());
    }

    #[test]
    fn test_feature_gated_type_attribute() {
        let ast = syn_build::attribute(
            quote! { #[cfg_attr(feature = "some_feature", node(numberTypeNode(u16, le)))] },
        );
        let attribute = NodeAttribute::parse(&ast).unwrap();

        assert_eq!(attribute.ast, &ast);
        assert_eq!(attribute.node, NumberTypeNode::le(U16).into());
    }
}
