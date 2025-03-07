use crate::utils::FromMeta;
use codama_nodes::BooleanValueNode;
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for BooleanValueNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        let value = meta.as_expr()?.as_literal_bool()?;
        Ok(BooleanValueNode::new(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_value;

    #[test]
    fn ok() {
        assert_value!({ true }, BooleanValueNode::new(true).into());
        assert_value!({ false }, BooleanValueNode::new(false).into());
    }
}
