use crate::get_path;
use codama::{Codama, NodeTrait};

#[test]
fn get_idl() {
    let codama = Codama::load(get_path("memo/crate")).unwrap();
    let idl = codama.get_idl().unwrap().to_json_pretty().unwrap();

    assert_eq!(
        idl,
        r#"{
  "kind": "rootNode",
  "standard": "codama",
  "version": "1.8.0",
  "program": {
    "kind": "programNode",
    "name": "memo",
    "publicKey": "MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr",
    "version": "1.0.0",
    "instructions": [
      {
        "kind": "instructionNode",
        "name": "addMemo",
        "arguments": [
          {
            "kind": "instructionArgumentNode",
            "name": "memo",
            "type": {
              "kind": "stringTypeNode",
              "encoding": "utf8"
            }
          }
        ],
        "remainingAccounts": [
          {
            "kind": "instructionRemainingAccountsNode",
            "isOptional": true,
            "isSigner": true,
            "docs": [
              "Expected signers of the memo."
            ],
            "value": {
              "kind": "argumentValueNode",
              "name": "signers"
            }
          }
        ]
      }
    ]
  }
}"#
    );
}
