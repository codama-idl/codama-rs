use crate::utils::SetOnce;
use codama_nodes::PublicKeyValueNode;
use codama_syn_helpers::{extensions::*, Meta};

/// Parses `program("token")` into the public key of a well-known program.
pub(crate) fn public_key_value_node_from_program(meta: &Meta) -> syn::Result<PublicKeyValueNode> {
    parse_well_known_public_key(meta, "program", "program_identifier", |name| {
        let public_key = match name {
            // Common SPL Programs
            "associated-token" => "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL",
            "memo" => "MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr",
            "memo-v1" => "Memo1UhkJRfHyvLMcVucJwxXeuD728EqVDDwQDxFMNo",
            "token" => "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
            "token-2022" => "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb",

            // Common Core Programs
            "config" => "Config1111111111111111111111111111111111111",
            "bpf-loader-upgradeable" => "BPFLoaderUpgradeab1e11111111111111111111111",
            "stake" => "Stake11111111111111111111111111111111111111",
            "system" => "11111111111111111111111111111111",
            "vote" => "Vote111111111111111111111111111111111111111",

            // Precompile Programs
            "ed25519" => "Ed25519SigVerify111111111111111111111111111",
            "secp256k1" => "KeccakSecp256k11111111111111111111111111111",
            "secp256r1" => "Secp256r1SigVerify1111111111111111111111111",
            _ => return None,
        };
        Some(public_key)
    })
}

/// Parses a `program("...")` or `sysvar("...")` value, whose single string `lookup` turns into a
/// public key. `kind` names the value in errors and `slot` names its string.
pub(crate) fn parse_well_known_public_key(
    meta: &Meta,
    kind: &str,
    slot: &'static str,
    lookup: fn(&str) -> Option<&'static str>,
) -> syn::Result<PublicKeyValueNode> {
    let pl = meta.as_path_list()?;
    let mut public_key = SetOnce::<String>::new(slot);
    pl.each(
        |ref meta| match lookup(meta.as_expr()?.as_string()?.as_str()) {
            Some(value) => public_key.set(value.to_string(), meta),
            None => Err(meta.error(format!("unrecognized {kind}"))),
        },
    )?;
    Ok(PublicKeyValueNode::new(public_key.take(meta)?))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::{assert_value, assert_value_err};

    #[test]
    fn ok() {
        assert_value!(
            { program("associated-token") },
            PublicKeyValueNode::new("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL").into()
        );
        assert_value!(
            { program("token") },
            PublicKeyValueNode::new("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").into()
        );
        assert_value!(
            { program("token-2022") },
            PublicKeyValueNode::new("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb").into()
        );
        assert_value!(
            { program("secp256r1") },
            PublicKeyValueNode::new("Secp256r1SigVerify1111111111111111111111111").into()
        );
        assert_value!(
            { program("system") },
            PublicKeyValueNode::new("11111111111111111111111111111111").into()
        );
    }

    #[test]
    fn invalid_input() {
        assert_value_err!({ program("invalid_program") }, "unrecognized program");
        assert_value_err!({ program(foo = 42) }, "expected a valid expression");
    }
}
