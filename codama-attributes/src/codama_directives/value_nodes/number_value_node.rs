use crate::utils::FromMeta;
use codama_nodes::{Number, NumberValueNode};
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for NumberValueNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        let expr = meta.as_expr()?;
        let value = expr
            .as_unsigned_integer::<u64>()
            .map(Number::from)
            .or(expr.as_signed_integer::<i64>().map(Number::from))
            .or(expr.as_float::<f64>().map(Number::from))?;
        Ok(NumberValueNode::new(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_value;

    #[test]
    fn ok() {
        assert_value!({ 42 }, NumberValueNode::new(42u64).into());
        assert_value!({ -42 }, NumberValueNode::new(-42i64).into());
        assert_value!({ 1.5 }, NumberValueNode::new(1.5f64).into());
        assert_value!({ -1.5 }, NumberValueNode::new(-1.5f64).into());
    }
}
