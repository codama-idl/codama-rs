use crate::utils::FromMeta;
use codama_nodes::{InstructionInputValueNode, PublicKeyValueNode, ValueNode};
use codama_syn_helpers::Meta;

impl FromMeta for InstructionInputValueNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        match meta.path_str().as_str() {
            "payer" => PublicKeyValueNode::from_meta(meta).map(Self::from), // TODO
            _ => ValueNode::from_meta(meta).map(Self::from),
        }
    }
}
