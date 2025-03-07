use crate::utils::{FromMeta, SetOnce};
use codama_nodes::PublicKeyValueNode;
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for PublicKeyValueNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        let pv = meta.as_path_list()?;
        let mut public_key = SetOnce::<String>::new("public_key");
        pv.each(|ref meta| {
            let value = meta.as_expr()?.as_string()?;
            public_key.set(value, meta)
        })?;
        Ok(Self::new(public_key.take(meta)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_value, assert_value_err};

    #[test]
    fn ok() {
        assert_value!(
            { public_key("1111") },
            PublicKeyValueNode::new("1111").into()
        );
        assert_value!(
            { public_key("2222") },
            PublicKeyValueNode::new("2222").into()
        );
    }

    #[test]
    fn invalid_input() {
        assert_value_err!({ public_key(unexpected) }, "expected a string");
        assert_value_err!({ public_key(foo = 42) }, "expected a valid expression");
    }
}
