use crate::utils::FromMeta;
use codama_nodes::PayerValueNode;
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for PayerValueNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        if !meta.is_path_or_empty_list() {
            return Err(meta.error("payer value does not accept any input"));
        }
        Ok(Self::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_value, assert_value_err};

    #[test]
    fn ok() {
        assert_value!({ payer }, PayerValueNode::new().into());
        assert_value!({ payer() }, PayerValueNode::new().into());
    }

    #[test]
    fn unexpected_input() {
        assert_value_err!(
            { payer(unexpected) },
            "payer value does not accept any input"
        );
        assert_value_err!({ payer(foo = 42) }, "payer value does not accept any input");
    }
}
