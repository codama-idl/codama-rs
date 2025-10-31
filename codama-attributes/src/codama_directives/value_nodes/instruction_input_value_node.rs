use crate::utils::FromMeta;
use codama_nodes::{
    AccountValueNode, ArgumentValueNode, InstructionInputValueNode, PayerValueNode, PdaValueNode,
    ValueNode,
};
use codama_syn_helpers::Meta;

impl FromMeta for InstructionInputValueNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        match meta.path_str().as_str() {
            "account" => AccountValueNode::from_meta(meta).map(Self::from),
            "argument" => ArgumentValueNode::from_meta(meta).map(Self::from),
            "payer" => PayerValueNode::from_meta(meta).map(Self::from),
            "pda" => PdaValueNode::from_meta(meta).map(Self::from),
            _ => ValueNode::from_meta(meta).map(Self::from),
        }
    }
}
