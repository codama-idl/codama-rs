use super::NodeAttributeParse;
use codama_nodes::*;
use codama_syn_helpers::{extensions::*, Meta};

impl NodeAttributeParse for Node {
    fn from_meta(meta: &Meta) -> syn::Result<Node> {
        let path = meta.path()?;
        match path.to_string().as_str() {
            // Type nodes.
            "boolean_type" => BooleanTypeNode::from_meta(meta),
            "fixed_size_type" => FixedSizeTypeNode::<TypeNode>::from_meta(meta),
            "number_type" => NumberTypeNode::from_meta(meta),
            "public_key_type" => PublicKeyTypeNode::from_meta(meta),
            _ => Err(path.error("unrecognized node")),
        }
    }
}
