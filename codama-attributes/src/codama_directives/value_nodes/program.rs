use crate::utils::SetOnce;
use codama_nodes::PublicKeyValueNode;
use codama_syn_helpers::{extensions::*, Meta};

pub fn public_key_value_node_from_program(meta: &Meta) -> syn::Result<PublicKeyValueNode> {
    let pv = meta.as_path_list()?;
    let mut program = SetOnce::<String>::new("program_identifier");
    pv.each(|ref meta| {
        let value = meta.as_expr()?.as_string()?;
        let public_key = match value.as_str() {
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

            _ => return Err(meta.error("unrecognized program")),
        };
        program.set(public_key.to_string(), meta)
    })?;
    Ok(PublicKeyValueNode::new(program.take(meta)?))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_value, assert_value_err};
    use codama_nodes::ProgramIdValueNode;

    #[test]
    fn with_identifier() {
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
    fn no_parenthesis() {
        assert_value!({ program }, ProgramIdValueNode::new().into());
    }

    #[test]
    fn empty_parenthesis() {
        assert_value!({ program() }, ProgramIdValueNode::new().into());
    }

    #[test]
    fn invalid_input() {
        assert_value_err!({ program("invalid_program") }, "unrecognized program");
        assert_value_err!({ program(foo = 42) }, "expected a valid expression");
    }
}
