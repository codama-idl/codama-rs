use crate::utils::FromMeta;
use codama_nodes::RemainderCountNode;
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for RemainderCountNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        meta.assert_directive("remainder_count")?;
        if meta.is_path_or_empty_list() {
            return Ok(RemainderCountNode::new());
        }
        Err(meta.error("remainder_count does not take any argument"))
    }
}
