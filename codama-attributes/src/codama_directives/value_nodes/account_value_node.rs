use crate::utils::{removed_name_key, FromMeta, SetOnce};
use codama_nodes::{AccountValueNode, Identifier};
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for AccountValueNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        Ok(AccountValueNode::new(parse_account_identifier(
            meta, "account",
        )?))
    }
}

/// Parses the account that a `directive("authority")` or `directive(identifier = "authority")` names,
/// such as `account` or `account_bump`.
pub(crate) fn parse_account_identifier(meta: &Meta, directive: &str) -> syn::Result<Identifier> {
    let pl = meta.assert_directive(directive)?.as_path_list()?;
    let mut identifier = SetOnce::<Identifier>::new("identifier");
    pl.each(|ref meta| match meta {
        Meta::PathValue(pv) if pv.path.is_strict("identifier") => {
            identifier.set(Identifier::from_meta(meta)?, meta)
        }
        Meta::PathValue(pv) if pv.path.is_strict("name") => removed_name_key(meta),
        Meta::PathValue(_) => Err(meta.error("unrecognized attribute")),
        _ => identifier.set(Identifier::from_meta(meta)?, meta),
    })?;
    identifier.take(meta)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::{assert_value, assert_value_err};

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
        assert_value_err!({ account(banana = "authority") }, "unrecognized attribute");
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
