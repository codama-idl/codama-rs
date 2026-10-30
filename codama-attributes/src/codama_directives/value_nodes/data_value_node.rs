use crate::utils::{field_path, FromMeta, SetOnce};
use codama_nodes::{DataValueNode, Path};
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for DataValueNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        let pl = meta.assert_directive("data")?.as_path_list()?;
        let mut path = SetOnce::<Path>::new("path");
        pl.each(|ref meta| match meta {
            Meta::Expr(expr) => path.set(field_path(&expr.as_string()?, expr)?, meta),
            _ => Err(meta.error("expected a string")),
        })?;
        Ok(DataValueNode::new(path.take(meta)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_value, assert_value_err};

    #[test]
    fn ok() {
        assert_value!(
            { data("amount") },
            DataValueNode::new(Path::try_from("amount").unwrap()).into()
        );
    }

    #[test]
    fn camel_cased_path() {
        assert_value!(
            { data("max_amount") },
            DataValueNode::new(Path::try_from("maxAmount").unwrap()).into()
        );
    }

    #[test]
    fn camel_cased_path_segments() {
        assert_value!(
            { data("config.fees[0]") },
            DataValueNode::new(Path::try_from("config.fees[0]").unwrap()).into()
        );
        assert_value!(
            { data("[0].my_field") },
            DataValueNode::new(Path::try_from("[0].myField").unwrap()).into()
        );
    }

    #[test]
    fn invalid_path() {
        assert_value_err!({ data("1st") }, "Invalid path: 1st");
        assert_value_err!({ data("a..b") }, "Invalid path: a..b");
    }

    #[test]
    fn expected_a_string() {
        assert_value_err!({ data(amount) }, "expected a string");
        assert_value_err!({ data(42) }, "expected a string");
        assert_value_err!({ data(name = "amount") }, "expected a string");
    }

    #[test]
    fn path_already_set() {
        assert_value_err!({ data("a", "b") }, "path is already set");
    }

    #[test]
    fn missing_path() {
        assert_value_err!({ data() }, "path is missing");
    }

    #[test]
    fn argument_is_removed() {
        assert_value_err!(
            { argument("amount") },
            "argument(...) is removed in v2, use data(...)"
        );
    }
}
