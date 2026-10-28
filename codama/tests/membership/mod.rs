use crate::get_path;
use codama::{Codama, NodeTrait};

#[test]
fn get_idl() {
    let codama = Codama::load(get_path("membership/crate")).unwrap();
    let idl = codama.get_idl().unwrap().to_json_pretty().unwrap();

    assert_eq!(
        idl,
        r#"{
  "kind": "rootNode",
  "standard": "codama",
  "version": "2.0.0",
  "program": {
    "kind": "programNode",
    "identifier": "membership",
    "publicKey": "Membership1111111111111111111111111",
    "version": "1.2.3",
    "accounts": [
      {
        "kind": "accountNode",
        "identifier": "person",
        "data": {
          "kind": "structTypeNode",
          "fields": [
            {
              "kind": "structFieldTypeNode",
              "identifier": "name",
              "type": {
                "kind": "stringTypeNode",
                "encoding": "utf8",
                "transforms": [
                  {
                    "kind": "sizePrefixTransformNode",
                    "prefix": {
                      "kind": "integerTypeNode",
                      "format": "u32",
                      "endian": "le"
                    }
                  }
                ]
              }
            },
            {
              "kind": "structFieldTypeNode",
              "identifier": "age",
              "type": {
                "kind": "integerTypeNode",
                "format": "u8",
                "endian": "le"
              }
            },
            {
              "kind": "structFieldTypeNode",
              "identifier": "membership",
              "type": {
                "kind": "definedTypeLinkNode",
                "identifier": "membership"
              }
            },
            {
              "kind": "structFieldTypeNode",
              "identifier": "wallet",
              "type": {
                "kind": "publicKeyTypeNode"
              }
            },
            {
              "kind": "structFieldTypeNode",
              "identifier": "bio",
              "type": {
                "kind": "definedTypeLinkNode",
                "identifier": "boundedString25"
              }
            }
          ]
        },
        "pda": {
          "kind": "pdaLinkNode",
          "identifier": "person"
        }
      }
    ],
    "definedTypes": [
      {
        "kind": "definedTypeNode",
        "identifier": "membership",
        "type": {
          "kind": "enumTypeNode",
          "variants": [
            {
              "kind": "enumVariantTypeNode",
              "identifier": "none"
            },
            {
              "kind": "enumVariantTypeNode",
              "identifier": "basic"
            },
            {
              "kind": "enumVariantTypeNode",
              "identifier": "premium"
            }
          ],
          "size": {
            "kind": "integerTypeNode",
            "format": "u8",
            "endian": "le"
          }
        }
      },
      {
        "kind": "definedTypeNode",
        "identifier": "boundedString25",
        "type": {
          "kind": "structTypeNode",
          "fields": [
            {
              "kind": "structFieldTypeNode",
              "identifier": "len",
              "type": {
                "kind": "integerTypeNode",
                "format": "u8",
                "endian": "le"
              }
            },
            {
              "kind": "structFieldTypeNode",
              "identifier": "value",
              "type": {
                "kind": "stringTypeNode",
                "encoding": "utf8",
                "transforms": [
                  {
                    "kind": "fixedSizeTransformNode",
                    "size": 25
                  }
                ]
              }
            }
          ]
        }
      }
    ],
    "pdas": [
      {
        "kind": "pdaNode",
        "identifier": "person",
        "seeds": [
          {
            "kind": "constantPdaSeedNode",
            "type": {
              "kind": "stringTypeNode",
              "encoding": "utf8"
            },
            "value": {
              "kind": "stringValueNode",
              "string": "person_pda"
            }
          },
          {
            "kind": "variablePdaSeedNode",
            "identifier": "wallet",
            "type": {
              "kind": "publicKeyTypeNode"
            }
          },
          {
            "kind": "variablePdaSeedNode",
            "identifier": "name",
            "type": {
              "kind": "stringTypeNode",
              "encoding": "utf8"
            }
          }
        ]
      }
    ]
  }
}"#
    );
}
