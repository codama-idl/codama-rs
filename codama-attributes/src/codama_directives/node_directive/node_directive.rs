use crate::{utils::SetOnce, FromMeta};
use codama_nodes::Node;
use codama_syn_helpers::{extensions::*, Meta};

#[derive(Debug, PartialEq)]
pub struct NodeDirective {
    pub node: Node,
}

impl TryFrom<&Meta> for NodeDirective {
    type Error = syn::Error;

    fn try_from(meta: &Meta) -> syn::Result<Self> {
        let list = meta.as_list()?;
        if !list.path.is_strict("node") {
            return Err(list.path.error("expected #[codama(node(...))]"));
        };

        let mut node = SetOnce::<Node>::new("node");
        list.each(|ref meta| node.set(Node::from_meta(meta)?, meta))?;
        Ok(Self {
            node: node.take(list)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use codama_nodes::{NumberFormat::U16, NumberTypeNode};
    use syn::parse_quote;

    #[test]
    fn single_input() {
        let meta: Meta = parse_quote! { node(number_type(u16, le)) };
        let node = NodeDirective::try_from(&meta).unwrap().node;

        assert_eq!(node, NumberTypeNode::le(U16).into());
    }

    #[test]
    fn no_input() {
        let meta: Meta = parse_quote! { node() };
        let error = NodeDirective::try_from(&meta).unwrap_err();
        assert!(error.to_string().contains("node is missing"));
    }

    #[test]
    fn multiple_inputs() {
        let meta: Meta = parse_quote! { node(number_type(u16, le), public_key_type()) };
        let error = NodeDirective::try_from(&meta).unwrap_err();
        assert!(error.to_string().contains("node is already set"));
    }

    #[test]
    fn unrecognized_attribute() {
        let meta: Meta = parse_quote! { node(wrongNode = 42) };
        let error = NodeDirective::try_from(&meta).unwrap_err();
        assert!(error.to_string().contains("unrecognized node"));
    }
}
