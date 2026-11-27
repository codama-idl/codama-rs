use super::program::parse_well_known_public_key;
use codama_nodes::PublicKeyValueNode;
use codama_syn_helpers::Meta;

/// Parses `sysvar("rent")` into the public key of a sysvar.
pub(crate) fn public_key_value_node_from_sysvar(meta: &Meta) -> syn::Result<PublicKeyValueNode> {
    parse_well_known_public_key(meta, "sysvar", "sysvar_identifier", |name| {
        let public_key = match name {
            "clock" => "SysvarC1ock11111111111111111111111111111111",
            "epoch_rewards" => "SysvarEpochRewards1111111111111111111111111",
            "epoch_schedule" => "SysvarEpochSchedu1e111111111111111111111111",
            "instructions" => "Sysvar1nstructions1111111111111111111111111",
            "last_restart_slot" => "SysvarLastRestartS1ot1111111111111111111111",
            "recent_blockhashes" => "SysvarRecentB1ockHashes11111111111111111111",
            "rent" => "SysvarRent111111111111111111111111111111111",
            "slot_hashes" => "SysvarS1otHashes111111111111111111111111111",
            "slot_history" => "SysvarS1otHistory11111111111111111111111111",
            "stake_history" => "SysvarStakeHistory1111111111111111111111111",
            _ => return None,
        };
        Some(public_key)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::{assert_value, assert_value_err};

    #[test]
    fn ok() {
        assert_value!(
            { sysvar("clock") },
            PublicKeyValueNode::new("SysvarC1ock11111111111111111111111111111111").into()
        );
        assert_value!(
            { sysvar("rent") },
            PublicKeyValueNode::new("SysvarRent111111111111111111111111111111111").into()
        );
    }

    #[test]
    fn invalid_input() {
        assert_value_err!({ sysvar("invalid_sysvar") }, "unrecognized sysvar");
        assert_value_err!({ sysvar(foo = 42) }, "expected a valid expression");
    }
}
