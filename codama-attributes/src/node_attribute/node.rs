use crate::NodeAttributeParse;
use codama_errors::CodamaResult;
use codama_nodes::{Node, NumberTypeNode};
use codama_syn_helpers::syn_traits::*;

impl NodeAttributeParse for Node {
    fn from_meta(meta: &syn::meta::ParseNestedMeta) -> CodamaResult<Node> {
        match meta.path.last_str().as_str() {
            "numberTypeNode" => NumberTypeNode::from_meta(&meta),
            _ => return Err(meta.error("unrecognized node").into()),
        }
    }
}
