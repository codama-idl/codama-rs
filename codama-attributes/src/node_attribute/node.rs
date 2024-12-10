use super::NodeAttributeParse;
use codama_nodes::*;
use codama_syn_helpers::syn_traits::*;

impl NodeAttributeParse for Node {
    fn from_meta(meta: &syn::meta::ParseNestedMeta) -> syn::Result<Node> {
        match meta.path.last_str().as_str() {
            // Type nodes.
            "boolean_type" => BooleanTypeNode::from_meta(&meta),
            "number_type" => NumberTypeNode::from_meta(&meta),
            "public_key_type" => PublicKeyTypeNode::from_meta(&meta),
            _ => return Err(meta.error("unrecognized node")),
        }
    }
}
