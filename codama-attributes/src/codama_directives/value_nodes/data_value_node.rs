use crate::utils::{removed_syntax, FromMeta, SetOnce};
use codama_nodes::{DataValueNode, Path};
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for DataValueNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        let pl = meta.assert_directive("data")?.as_path_list()?;
        let mut path = SetOnce::<Path>::new("path");
        pl.each(|ref meta| match meta {
            Meta::Expr(expr) => path.set(
                Path::try_from(expr.as_string()?).map_err(|e| expr.error(e))?,
                meta,
            ),
            // The v1 `argument(name = "x")` spelling, carried over to its v2 replacement.
            Meta::PathValue(pv) if pv.path.is_strict("name") => {
                removed_syntax(meta, "name = ...", "data(\"...\")")
            }
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
        assert_value!({ data("amount") }, DataValueNode::new("amount").into());
    }

    #[test]
    fn path_as_written() {
        assert_value!(
            { data("max_amount") },
            DataValueNode::new("max_amount").into()
        );
        assert_value!(
            { data("maxAmount") },
            DataValueNode::new("maxAmount").into()
        );
    }

    #[test]
    fn path_segments_as_written() {
        assert_value!(
            { data("config.fees[0]") },
            DataValueNode::new("config.fees[0]").into()
        );
        assert_value!(
            { data("[0].my_field") },
            DataValueNode::new("[0].my_field").into()
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
        assert_value_err!({ data(identifier = "amount") }, "expected a string");
    }

    #[test]
    fn removed_name_key() {
        assert_value_err!(
            { data(name = "amount") },
            "name = ... is removed in v2, use data(\"...\")"
        );
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
