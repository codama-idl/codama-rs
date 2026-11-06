use crate::utils::{removed_syntax, FromMeta};
use codama_nodes::{
    AccountBumpValueNode, AccountValueNode, DataValueNode, InstructionInputValueNode,
    PayerValueNode, PdaValueNode, ValueNode,
};
use codama_syn_helpers::Meta;

impl FromMeta for InstructionInputValueNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        match meta.path_str().as_str() {
            "account" => AccountValueNode::from_meta(meta).map(Self::from),
            "account_bump" => AccountBumpValueNode::from_meta(meta).map(Self::from),
            "argument" => removed_syntax(meta, "argument(...)", "data(...)"),
            "data" => DataValueNode::from_meta(meta).map(Self::from),
            "payer" => PayerValueNode::from_meta(meta).map(Self::from),
            "pda" => PdaValueNode::from_meta(meta).map(Self::from),
            _ => ValueNode::from_meta(meta).map(Self::from),
        }
    }
}
