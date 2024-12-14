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
    "name": "membership",
    "publicKey": "Membership1111111111111111111111111",
    "version": "1.2.3",
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
      },
      {
        "kind": "definedTypeNode",
        "name": "person",
        "type": {
          "kind": "structTypeNode",
          "fields": [
            {
              "kind": "structFieldTypeNode",
              "name": "name",
              "type": {
                "kind": "sizePrefixTypeNode",
                "type": {
                  "kind": "stringTypeNode",
                  "encoding": "utf8"
                },
                "prefix": {
                  "kind": "numberTypeNode",
                  "format": "u32",
                  "endian": "le"
                }
              }
            },
            {
              "kind": "structFieldTypeNode",
              "name": "age",
              "type": {
                "kind": "numberTypeNode",
                "format": "u8",
                "endian": "le"
              }
            },
            {
              "kind": "structFieldTypeNode",
              "name": "membership",
              "type": {
                "kind": "definedTypeLinkNode",
                "name": "membership"
              }
            },
            {
              "kind": "structFieldTypeNode",
              "name": "wallet",
              "type": {
                "kind": "publicKeyTypeNode"
              }
            }
          ]
        }
      }
    ]
  },
  "additionalPrograms": []
}"#
    );
}
