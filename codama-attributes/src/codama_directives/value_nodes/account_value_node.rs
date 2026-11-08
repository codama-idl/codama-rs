use crate::utils::{removed_name_key, FromMeta, SetOnce};
use codama_nodes::{AccountValueNode, Identifier};
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for AccountValueNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        let pl = meta.assert_directive("account")?.as_path_list()?;
        let mut name = SetOnce::<String>::new("identifier");
        pl.each(|ref meta| match meta {
            Meta::PathValue(pv) => {
                if pv.path.is_strict("name") {
                    return removed_name_key(meta);
                }
                if !pv.path.is_strict("identifier") {
                    return Err(pv.path.error("only 'identifier' attribute supported"));
                };
                name.set(pv.value.as_expr()?.as_string()?, meta)
            }
            _ => name.set(meta.as_expr()?.as_string()?, meta),
        })?;

        let name = name.take(meta)?;
        Ok(AccountValueNode::new(
            name.parse::<Identifier>().map_err(|e| meta.error(e))?,
        ))
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
            { account(identifier = "authority") },
            AccountValueNode::new("authority").into()
        );
    }

    #[test]
    fn wrong_name_attribute() {
        assert_value_err!(
            { account(banana = "authority") },
            "only 'identifier' attribute supported"
        );
    }

    #[test]
    fn missing_name() {
        assert_value_err!({ account() }, "identifier is missing");
    }

    #[test]
    fn name_is_removed() {
        assert_value_err!(
            { account(name = "authority") },
            "name = ... is removed in v2, use identifier = ..."
        );
    }
}
