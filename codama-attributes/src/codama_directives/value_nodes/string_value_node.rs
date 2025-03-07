use crate::utils::FromMeta;
use codama_nodes::StringValueNode;
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for StringValueNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        let value = meta.as_expr()?.as_string()?;
        Ok(StringValueNode::new(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_value;

    #[test]
    fn ok() {
        assert_value!({ "hello" }, StringValueNode::new("hello").into());
        assert_value!({ "world" }, StringValueNode::new("world").into());
    }
}
