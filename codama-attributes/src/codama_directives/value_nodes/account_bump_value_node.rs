use super::account_value_node::parse_account_identifier;
use crate::utils::FromMeta;
use codama_nodes::AccountBumpValueNode;
use codama_syn_helpers::Meta;

impl FromMeta for AccountBumpValueNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        Ok(AccountBumpValueNode::new(parse_account_identifier(
            meta,
            "account_bump",
        )?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::{assert_value, assert_value_err};

    #[test]
    fn ok() {
        assert_value!(
            { account_bump("escrow") },
            AccountBumpValueNode::new("escrow").into()
        );
        assert_value!(
            { account_bump(identifier = "escrow") },
            AccountBumpValueNode::new("escrow").into()
        );
    }

    #[test]
    fn wrong_name_attribute() {
        assert_value_err!(
            { account_bump(banana = "escrow") },
            "unrecognized attribute"
        );
    }

    #[test]
    fn missing_name() {
        assert_value_err!({ account_bump() }, "identifier is missing");
    }

    #[test]
    fn name_is_removed() {
        assert_value_err!(
            { account_bump(name = "authority") },
            "name = ... is removed in v2, use identifier = ..."
        );
    }
}
