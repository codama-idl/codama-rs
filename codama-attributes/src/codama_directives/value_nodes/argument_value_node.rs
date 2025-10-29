use crate::utils::{FromMeta, SetOnce};
use codama_nodes::ArgumentValueNode;
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for ArgumentValueNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        let pl = meta.assert_directive("argument")?.as_path_list()?;
        let mut name = SetOnce::<String>::new("name");
        pl.each(|ref meta| match meta {
            Meta::PathValue(pv) => {
                if !pv.path.is_strict("name") {
                    return Err(pv.path.error("only 'name' attribute supported"));
                };
                name.set(String::from_meta(meta)?, meta)
            }
            _ => name.set(String::from_meta(meta)?, meta),
        })?;

        Ok(ArgumentValueNode::new(name.take(meta)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_value, assert_value_err};

    #[test]
    fn ok() {
        assert_value!(
            { argument("amount") },
            ArgumentValueNode::new("amount").into()
        );
        assert_value!(
            { argument(name = "amount") },
            ArgumentValueNode::new("amount").into()
        );
    }

    #[test]
    fn wrong_name_attribute() {
        assert_value_err!(
            { argument(banana = "amount") },
            "only 'name' attribute supported"
        );
    }

    #[test]
    fn missing_name() {
        assert_value_err!({ argument() }, "name is missing");
    }
}
