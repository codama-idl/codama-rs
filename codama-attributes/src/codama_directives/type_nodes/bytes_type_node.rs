use crate::utils::FromMeta;
use codama_nodes::BytesTypeNode;
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for BytesTypeNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        meta.assert_directive("bytes")?;
        if !meta.is_path_or_empty_list() {
            return Err(meta.error("bytes does not accept any input"));
        }
        Ok(Self::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_type, assert_type_err};

    #[test]
    fn ok() {
        assert_type!({ bytes }, BytesTypeNode::new().into());
        assert_type!({ bytes() }, BytesTypeNode::new().into());
    }

    #[test]
    fn unexpected_input() {
        assert_type_err!({ bytes(unexpected) }, "bytes does not accept any input");
        assert_type_err!({ bytes(foo = 42) }, "bytes does not accept any input");
    }
}
