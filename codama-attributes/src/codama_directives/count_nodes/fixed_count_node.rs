use crate::utils::{FromMeta, SetOnce};
use codama_nodes::FixedCountNode;
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for FixedCountNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        let pl = meta.assert_directive("fixed_count")?.as_path_list()?;
        let mut value: SetOnce<u64> = SetOnce::new("value");

        pl.each(|ref meta| match meta.path_str().as_str() {
            "value" => value.set(meta.as_value()?.as_expr()?.as_unsigned_integer()?, meta),
            _ => {
                if let Ok(expr) = meta.as_expr() {
                    return value.set(expr.as_unsigned_integer()?, meta);
                }
                Err(meta.error("unrecognized attribute"))
            }
        })?;

        Ok(FixedCountNode::new(value.take(meta)?))
    }
}
