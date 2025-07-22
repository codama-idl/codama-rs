use crate::codama_directives::value_nodes::{
    program::public_key_value_node_from_program, sysvar::public_key_value_node_from_sysvar,
};
use crate::utils::FromMeta;
use codama_nodes::{
    BooleanValueNode, NumberValueNode, ProgramIdValueNode, PublicKeyValueNode, StringValueNode,
    ValueNode,
};
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for ValueNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        match meta.path_str().as_str() {
            "public_key" => PublicKeyValueNode::from_meta(meta).map(Self::from),
            "program" => {
                // If it's just `program` (no parenthesis or empty parenthesis),
                // return `ProgramIdValueNode`.
                if meta.is_path_or_empty_list() {
                    Ok(ProgramIdValueNode::new().into())
                } else {
                    // Otherwise, parse the program identifier and return
                    // `PublicKeyValueNode`.
                    public_key_value_node_from_program(meta).map(Self::from)
                }
            }
            "sysvar" => public_key_value_node_from_sysvar(meta).map(Self::from),
            _ => BooleanValueNode::from_meta(meta)
                .map(Self::from)
                .or(StringValueNode::from_meta(meta).map(Self::from))
                .or(NumberValueNode::from_meta(meta).map(Self::from))
                .map_err(|_| meta.error("unrecognized value")),
        }
    }
}
