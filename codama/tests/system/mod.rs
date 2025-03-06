use crate::get_path;
use codama::{Codama, NodeTrait};

#[test]
fn get_idl() {
    let codama = Codama::load(get_path("system/crate")).unwrap();
    let idl = codama.get_idl().unwrap().to_json_pretty().unwrap();

    assert_eq!(
        idl,
        r#"{
  "kind": "rootNode",
  "standard": "codama",
  "version": "1.0.0",
  "program": {
    "kind": "programNode",
    "name": "system",
    "publicKey": "11111111111111111111111111111111",
    "version": "1.0.0",
    "accounts": [
      {
        "kind": "accountNode",
        "name": "nonce",
        "data": {
          "kind": "structTypeNode",
          "fields": [
            {
              "kind": "structFieldTypeNode",
              "name": "version",
              "type": {
                "kind": "definedTypeLinkNode",
                "name": "nonceVersion"
              }
            },
            {
              "kind": "structFieldTypeNode",
              "name": "state",
              "type": {
                "kind": "definedTypeLinkNode",
                "name": "nonceState"
              }
            },
            {
              "kind": "structFieldTypeNode",
              "name": "authority",
              "type": {
                "kind": "publicKeyTypeNode"
              }
            },
            {
              "kind": "structFieldTypeNode",
              "name": "blockhash",
              "type": {
                "kind": "publicKeyTypeNode"
              }
            },
            {
              "kind": "structFieldTypeNode",
              "name": "lamportsPerSignature",
              "type": {
                "kind": "numberTypeNode",
                "format": "u64",
                "endian": "le"
              }
            }
          ]
        }
      }
    ],
    "instructions": [
      {
        "kind": "instructionNode",
        "name": "createAccount",
        "accounts": [
          {
            "kind": "instructionAccountNode",
            "name": "payer",
            "isWritable": true,
            "isSigner": true
          },
          {
            "kind": "instructionAccountNode",
            "name": "newAccount",
            "isWritable": true,
            "isSigner": true
          }
        ],
        "arguments": [
          {
            "kind": "instructionArgumentNode",
            "name": "discriminator",
            "defaultValueStrategy": "omitted",
            "type": {
              "kind": "numberTypeNode",
              "format": "u32",
              "endian": "le"
            },
            "defaultValue": {
              "kind": "numberValueNode",
              "number": 0
            }
          },
          {
            "kind": "instructionArgumentNode",
            "name": "lamports",
            "type": {
              "kind": "numberTypeNode",
              "format": "u64",
              "endian": "le"
            }
          },
          {
            "kind": "instructionArgumentNode",
            "name": "space",
            "type": {
              "kind": "numberTypeNode",
              "format": "u64",
              "endian": "le"
            }
          },
          {
            "kind": "instructionArgumentNode",
            "name": "programAddress",
            "type": {
              "kind": "publicKeyTypeNode"
            }
          }
        ],
        "discriminators": [
          {
            "kind": "fieldDiscriminatorNode",
            "name": "discriminator",
            "offset": 0
          }
        ]
      },
      {
        "kind": "instructionNode",
        "name": "assign",
        "accounts": [
          {
            "kind": "instructionAccountNode",
            "name": "account",
            "isWritable": true,
            "isSigner": true
          }
        ],
        "arguments": [
          {
            "kind": "instructionArgumentNode",
            "name": "discriminator",
            "defaultValueStrategy": "omitted",
            "type": {
              "kind": "numberTypeNode",
              "format": "u32",
              "endian": "le"
            },
            "defaultValue": {
              "kind": "numberValueNode",
              "number": 1
            }
          },
          {
            "kind": "instructionArgumentNode",
            "name": "programAddress",
            "type": {
              "kind": "publicKeyTypeNode"
            }
          }
        ],
        "discriminators": [
          {
            "kind": "fieldDiscriminatorNode",
            "name": "discriminator",
            "offset": 0
          }
        ]
      },
      {
        "kind": "instructionNode",
        "name": "transferSol",
        "accounts": [
          {
            "kind": "instructionAccountNode",
            "name": "source",
            "isWritable": true,
            "isSigner": true
          },
          {
            "kind": "instructionAccountNode",
            "name": "destination",
            "isWritable": true,
            "isSigner": false
          }
        ],
        "arguments": [
          {
            "kind": "instructionArgumentNode",
            "name": "discriminator",
            "defaultValueStrategy": "omitted",
            "type": {
              "kind": "numberTypeNode",
              "format": "u32",
              "endian": "le"
            },
            "defaultValue": {
              "kind": "numberValueNode",
              "number": 2
            }
          },
          {
            "kind": "instructionArgumentNode",
            "name": "amount",
            "type": {
              "kind": "numberTypeNode",
              "format": "u64",
              "endian": "le"
            }
          }
        ],
        "discriminators": [
          {
            "kind": "fieldDiscriminatorNode",
            "name": "discriminator",
            "offset": 0
          }
        ]
      },
      {
        "kind": "instructionNode",
        "name": "createAccountWithSeed",
        "accounts": [
          {
            "kind": "instructionAccountNode",
            "name": "payer",
            "isWritable": true,
            "isSigner": true
          },
          {
            "kind": "instructionAccountNode",
            "name": "newAccount",
            "isWritable": true,
            "isSigner": false
          },
          {
            "kind": "instructionAccountNode",
            "name": "baseAccount",
            "isWritable": false,
            "isSigner": true
          }
        ],
        "arguments": [
          {
            "kind": "instructionArgumentNode",
            "name": "discriminator",
            "defaultValueStrategy": "omitted",
            "type": {
              "kind": "numberTypeNode",
              "format": "u32",
              "endian": "le"
            },
            "defaultValue": {
              "kind": "numberValueNode",
              "number": 3
            }
          },
          {
            "kind": "instructionArgumentNode",
            "name": "base",
            "type": {
              "kind": "publicKeyTypeNode"
            }
          },
          {
            "kind": "instructionArgumentNode",
            "name": "seed",
            "type": {
              "kind": "sizePrefixTypeNode",
              "type": {
                "kind": "stringTypeNode",
                "encoding": "utf8"
              },
              "prefix": {
                "kind": "numberTypeNode",
                "format": "u64",
                "endian": "le"
              }
            }
          },
          {
            "kind": "instructionArgumentNode",
            "name": "amount",
            "type": {
              "kind": "numberTypeNode",
              "format": "u64",
              "endian": "le"
            }
          },
          {
            "kind": "instructionArgumentNode",
            "name": "space",
            "type": {
              "kind": "numberTypeNode",
              "format": "u64",
              "endian": "le"
            }
          },
          {
            "kind": "instructionArgumentNode",
            "name": "programAddress",
            "type": {
              "kind": "publicKeyTypeNode"
            }
          }
        ],
        "discriminators": [
          {
            "kind": "fieldDiscriminatorNode",
            "name": "discriminator",
            "offset": 0
          }
        ]
      },
      {
        "kind": "instructionNode",
        "name": "advanceNonceAccount",
        "accounts": [
          {
            "kind": "instructionAccountNode",
            "name": "nonceAccount",
            "isWritable": true,
            "isSigner": false
          },
          {
            "kind": "instructionAccountNode",
            "name": "recentBlockhashesSysvar",
            "isWritable": false,
            "isSigner": false
          },
          {
            "kind": "instructionAccountNode",
            "name": "nonceAuthority",
            "isWritable": false,
            "isSigner": true
          }
        ],
        "arguments": [
          {
            "kind": "instructionArgumentNode",
            "name": "discriminator",
            "defaultValueStrategy": "omitted",
            "type": {
              "kind": "numberTypeNode",
              "format": "u32",
              "endian": "le"
            },
            "defaultValue": {
              "kind": "numberValueNode",
              "number": 4
            }
          }
        ],
        "discriminators": [
          {
            "kind": "fieldDiscriminatorNode",
            "name": "discriminator",
            "offset": 0
          }
        ]
      },
      {
        "kind": "instructionNode",
        "name": "withdrawNonceAccount",
        "accounts": [
          {
            "kind": "instructionAccountNode",
            "name": "nonceAccount",
            "isWritable": true,
            "isSigner": false
          },
          {
            "kind": "instructionAccountNode",
            "name": "recipientAccount",
            "isWritable": true,
            "isSigner": false
          },
          {
            "kind": "instructionAccountNode",
            "name": "recentBlockhashesSysvar",
            "isWritable": false,
            "isSigner": false
          },
          {
            "kind": "instructionAccountNode",
            "name": "rentSysvar",
            "isWritable": false,
            "isSigner": false
          },
          {
            "kind": "instructionAccountNode",
            "name": "nonceAuthority",
            "isWritable": false,
            "isSigner": true
          }
        ],
        "arguments": [
          {
            "kind": "instructionArgumentNode",
            "name": "discriminator",
            "defaultValueStrategy": "omitted",
            "type": {
              "kind": "numberTypeNode",
              "format": "u32",
              "endian": "le"
            },
            "defaultValue": {
              "kind": "numberValueNode",
              "number": 5
            }
          },
          {
            "kind": "instructionArgumentNode",
            "name": "withdrawAmount",
            "type": {
              "kind": "numberTypeNode",
              "format": "u64",
              "endian": "le"
            }
          }
        ],
        "discriminators": [
          {
            "kind": "fieldDiscriminatorNode",
            "name": "discriminator",
            "offset": 0
          }
        ]
      },
      {
        "kind": "instructionNode",
        "name": "initializeNonceAccount",
        "accounts": [
          {
            "kind": "instructionAccountNode",
            "name": "nonceAccount",
            "isWritable": true,
            "isSigner": false
          },
          {
            "kind": "instructionAccountNode",
            "name": "recentBlockhashesSysvar",
            "isWritable": false,
            "isSigner": false
          },
          {
            "kind": "instructionAccountNode",
            "name": "rentSysvar",
            "isWritable": false,
            "isSigner": false
          }
        ],
        "arguments": [
          {
            "kind": "instructionArgumentNode",
            "name": "discriminator",
            "defaultValueStrategy": "omitted",
            "type": {
              "kind": "numberTypeNode",
              "format": "u32",
              "endian": "le"
            },
            "defaultValue": {
              "kind": "numberValueNode",
              "number": 6
            }
          },
          {
            "kind": "instructionArgumentNode",
            "name": "nonceAuthority",
            "type": {
              "kind": "publicKeyTypeNode"
            }
          }
        ],
        "discriminators": [
          {
            "kind": "fieldDiscriminatorNode",
            "name": "discriminator",
            "offset": 0
          }
        ]
      },
      {
        "kind": "instructionNode",
        "name": "authorizeNonceAccount",
        "accounts": [
          {
            "kind": "instructionAccountNode",
            "name": "nonceAccount",
            "isWritable": true,
            "isSigner": false
          },
          {
            "kind": "instructionAccountNode",
            "name": "nonceAuthority",
            "isWritable": false,
            "isSigner": true
          }
        ],
        "arguments": [
          {
            "kind": "instructionArgumentNode",
            "name": "discriminator",
            "defaultValueStrategy": "omitted",
            "type": {
              "kind": "numberTypeNode",
              "format": "u32",
              "endian": "le"
            },
            "defaultValue": {
              "kind": "numberValueNode",
              "number": 7
            }
          },
          {
            "kind": "instructionArgumentNode",
            "name": "newNonceAuthority",
            "type": {
              "kind": "publicKeyTypeNode"
            }
          }
        ],
        "discriminators": [
          {
            "kind": "fieldDiscriminatorNode",
            "name": "discriminator",
            "offset": 0
          }
        ]
      },
      {
        "kind": "instructionNode",
        "name": "allocate",
        "accounts": [
          {
            "kind": "instructionAccountNode",
            "name": "newAccount",
            "isWritable": true,
            "isSigner": true
          }
        ],
        "arguments": [
          {
            "kind": "instructionArgumentNode",
            "name": "discriminator",
            "defaultValueStrategy": "omitted",
            "type": {
              "kind": "numberTypeNode",
              "format": "u32",
              "endian": "le"
            },
            "defaultValue": {
              "kind": "numberValueNode",
              "number": 8
            }
          },
          {
            "kind": "instructionArgumentNode",
            "name": "space",
            "type": {
              "kind": "numberTypeNode",
              "format": "u64",
              "endian": "le"
            }
          }
        ],
        "discriminators": [
          {
            "kind": "fieldDiscriminatorNode",
            "name": "discriminator",
            "offset": 0
          }
        ]
      },
      {
        "kind": "instructionNode",
        "name": "allocateWithSeed",
        "accounts": [
          {
            "kind": "instructionAccountNode",
            "name": "newAccount",
            "isWritable": true,
            "isSigner": false
          },
          {
            "kind": "instructionAccountNode",
            "name": "baseAccount",
            "isWritable": false,
            "isSigner": true
          }
        ],
        "arguments": [
          {
            "kind": "instructionArgumentNode",
            "name": "discriminator",
            "defaultValueStrategy": "omitted",
            "type": {
              "kind": "numberTypeNode",
              "format": "u32",
              "endian": "le"
            },
            "defaultValue": {
              "kind": "numberValueNode",
              "number": 9
            }
          },
          {
            "kind": "instructionArgumentNode",
            "name": "base",
            "type": {
              "kind": "publicKeyTypeNode"
            }
          },
          {
            "kind": "instructionArgumentNode",
            "name": "seed",
            "type": {
              "kind": "sizePrefixTypeNode",
              "type": {
                "kind": "stringTypeNode",
                "encoding": "utf8"
              },
              "prefix": {
                "kind": "numberTypeNode",
                "format": "u64",
                "endian": "le"
              }
            }
          },
          {
            "kind": "instructionArgumentNode",
            "name": "space",
            "type": {
              "kind": "numberTypeNode",
              "format": "u64",
              "endian": "le"
            }
          },
          {
            "kind": "instructionArgumentNode",
            "name": "programAddress",
            "type": {
              "kind": "publicKeyTypeNode"
            }
          }
        ],
        "discriminators": [
          {
            "kind": "fieldDiscriminatorNode",
            "name": "discriminator",
            "offset": 0
          }
        ]
      },
      {
        "kind": "instructionNode",
        "name": "assignWithSeed",
        "accounts": [
          {
            "kind": "instructionAccountNode",
            "name": "account",
            "isWritable": true,
            "isSigner": false
          },
          {
            "kind": "instructionAccountNode",
            "name": "baseAccount",
            "isWritable": false,
            "isSigner": true
          }
        ],
        "arguments": [
          {
            "kind": "instructionArgumentNode",
            "name": "discriminator",
            "defaultValueStrategy": "omitted",
            "type": {
              "kind": "numberTypeNode",
              "format": "u32",
              "endian": "le"
            },
            "defaultValue": {
              "kind": "numberValueNode",
              "number": 10
            }
          },
          {
            "kind": "instructionArgumentNode",
            "name": "base",
            "type": {
              "kind": "publicKeyTypeNode"
            }
          },
          {
            "kind": "instructionArgumentNode",
            "name": "seed",
            "type": {
              "kind": "sizePrefixTypeNode",
              "type": {
                "kind": "stringTypeNode",
                "encoding": "utf8"
              },
              "prefix": {
                "kind": "numberTypeNode",
                "format": "u64",
                "endian": "le"
              }
            }
          },
          {
            "kind": "instructionArgumentNode",
            "name": "programAddress",
            "type": {
              "kind": "publicKeyTypeNode"
            }
          }
        ],
        "discriminators": [
          {
            "kind": "fieldDiscriminatorNode",
            "name": "discriminator",
            "offset": 0
          }
        ]
      },
      {
        "kind": "instructionNode",
        "name": "transferSolWithSeed",
        "accounts": [
          {
            "kind": "instructionAccountNode",
            "name": "source",
            "isWritable": true,
            "isSigner": false
          },
          {
            "kind": "instructionAccountNode",
            "name": "baseAccount",
            "isWritable": false,
            "isSigner": true
          },
          {
            "kind": "instructionAccountNode",
            "name": "destination",
            "isWritable": true,
            "isSigner": false
          }
        ],
        "arguments": [
          {
            "kind": "instructionArgumentNode",
            "name": "discriminator",
            "defaultValueStrategy": "omitted",
            "type": {
              "kind": "numberTypeNode",
              "format": "u32",
              "endian": "le"
            },
            "defaultValue": {
              "kind": "numberValueNode",
              "number": 11
            }
          },
          {
            "kind": "instructionArgumentNode",
            "name": "amount",
            "type": {
              "kind": "numberTypeNode",
              "format": "u64",
              "endian": "le"
            }
          },
          {
            "kind": "instructionArgumentNode",
            "name": "fromSeed",
            "type": {
              "kind": "sizePrefixTypeNode",
              "type": {
                "kind": "stringTypeNode",
                "encoding": "utf8"
              },
              "prefix": {
                "kind": "numberTypeNode",
                "format": "u64",
                "endian": "le"
              }
            }
          },
          {
            "kind": "instructionArgumentNode",
            "name": "fromOwner",
            "type": {
              "kind": "publicKeyTypeNode"
            }
          }
        ],
        "discriminators": [
          {
            "kind": "fieldDiscriminatorNode",
            "name": "discriminator",
            "offset": 0
          }
        ]
      },
      {
        "kind": "instructionNode",
        "name": "upgradeNonceAccount",
        "accounts": [
          {
            "kind": "instructionAccountNode",
            "name": "nonceAccount",
            "isWritable": true,
            "isSigner": false
          }
        ],
        "arguments": [
          {
            "kind": "instructionArgumentNode",
            "name": "discriminator",
            "defaultValueStrategy": "omitted",
            "type": {
              "kind": "numberTypeNode",
              "format": "u32",
              "endian": "le"
            },
            "defaultValue": {
              "kind": "numberValueNode",
              "number": 12
            }
          }
        ],
        "discriminators": [
          {
            "kind": "fieldDiscriminatorNode",
            "name": "discriminator",
            "offset": 0
          }
        ]
      }
    ],
    "definedTypes": [
      {
        "kind": "definedTypeNode",
        "name": "nonceVersion",
        "type": {
          "kind": "enumTypeNode",
          "variants": [
            {
              "kind": "enumEmptyVariantTypeNode",
              "name": "legacy"
            },
            {
              "kind": "enumEmptyVariantTypeNode",
              "name": "current"
            }
          ],
          "size": {
            "kind": "numberTypeNode",
            "format": "u32",
            "endian": "le"
          }
        }
      },
      {
        "kind": "definedTypeNode",
        "name": "nonceState",
        "type": {
          "kind": "enumTypeNode",
          "variants": [
            {
              "kind": "enumEmptyVariantTypeNode",
              "name": "uninitialized"
            },
            {
              "kind": "enumEmptyVariantTypeNode",
              "name": "initialized"
            }
          ],
          "size": {
            "kind": "numberTypeNode",
            "format": "u32",
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
