use crate::get_path;
use codama::{Codama, NodeTrait};

#[test]
fn get_idl() {
    let codama = Codama::load(&get_path("membership/crate")).unwrap();
    let idl = codama.get_idl().unwrap().to_json_pretty().unwrap();

    assert_eq!(
        idl,
        r#"{
  "kind": "rootNode",
  "program": {
    "kind": "programNode",
    "name": "",
    "publicKey": "",
    "version": "",
    "accounts": [],
    "instructions": [],
    "definedTypes": [
      {
        "kind": "definedTypeNode",
        "name": "membership",
        "type": {
          "kind": "enumTypeNode",
          "variants": [
            {
              "kind": "enumEmptyVariantTypeNode",
              "name": "none"
            },
            {
              "kind": "enumEmptyVariantTypeNode",
              "name": "basic"
            },
            {
              "kind": "enumEmptyVariantTypeNode",
              "name": "premium"
            }
          ],
          "size": {
            "kind": "numberTypeNode",
            "format": "u8",
            "endian": "le"
          }
        }
      }
    ]
  },
  "additionalPrograms": []
}"#
    );
}
