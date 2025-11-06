use crate::utils::{FromMeta, SetOnce};
use codama_nodes::AccountValueNode;
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for AccountValueNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        let pl = meta.assert_directive("account")?.as_path_list()?;
        let mut name = SetOnce::<String>::new("name");
        pl.each(|ref meta| match meta {
            Meta::PathValue(pv) => {
                if !pv.path.is_strict("name") {
                    return Err(pv.path.error("only 'name' attribute supported"));
                };
                name.set(pv.value.as_expr()?.as_string()?, meta)
            }
            _ => name.set(meta.as_expr()?.as_string()?, meta),
        })?;

        Ok(AccountValueNode::new(name.take(meta)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_value, assert_value_err};

    #[test]
    fn ok() {
        assert_value!(
            { account("authority") },
            AccountValueNode::new("authority").into()
        );
        assert_value!(
            { account(name = "authority") },
            AccountValueNode::new("authority").into()
        );
    }

    #[test]
    fn wrong_name_attribute() {
        assert_value_err!(
            { account(banana = "authority") },
            "only 'name' attribute supported"
        );
    }

    #[test]
    fn missing_name() {
        assert_value_err!({ account() }, "name is missing");
    }
}
