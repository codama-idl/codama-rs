use super::NodeAttributeParse;
use codama_nodes::*;
use codama_syn_helpers::syn_traits::*;

impl NodeAttributeParse for Node {
    fn from_meta(path: &syn::Path, meta: &syn::meta::ParseNestedMeta) -> syn::Result<Node> {
        match path.last_str().as_str() {
            // Type nodes.
            "boolean_type" => BooleanTypeNode::from_meta(path, meta),
            "number_type" => NumberTypeNode::from_meta(path, meta),
            "public_key_type" => PublicKeyTypeNode::from_meta(path, meta),
            _ => return Err(meta.error("unrecognized node")),
        }
    }
}
