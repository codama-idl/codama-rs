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
  "version": "2.0.0",
  "program": {
    "kind": "programNode",
    "identifier": "system",
    "publicKey": "11111111111111111111111111111111",
    "version": "1.0.0",
    "accounts": [
      {
        "kind": "accountNode",
        "identifier": "nonce",
        "data": {
          "kind": "structTypeNode",
          "fields": [
            {
              "kind": "structFieldTypeNode",
              "identifier": "version",
              "type": {
                "kind": "definedTypeLinkNode",
                "identifier": "nonceVersion"
              }
            },
            {
              "kind": "structFieldTypeNode",
              "identifier": "state",
              "type": {
                "kind": "definedTypeLinkNode",
                "identifier": "nonceState"
              }
            },
            {
              "kind": "structFieldTypeNode",
              "identifier": "authority",
              "type": {
                "kind": "publicKeyTypeNode"
              }
            },
            {
              "kind": "structFieldTypeNode",
              "identifier": "blockhash",
              "type": {
                "kind": "publicKeyTypeNode"
              }
            },
            {
              "kind": "structFieldTypeNode",
              "identifier": "lamportsPerSignature",
              "type": {
                "kind": "integerTypeNode",
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
        "identifier": "createAccount",
        "accounts": [
          {
            "kind": "instructionAccountNode",
            "identifier": "payer",
            "isWritable": true,
            "isSigner": true,
            "defaultValue": {
              "kind": "payerValueNode"
            }
          },
          {
            "kind": "instructionAccountNode",
            "identifier": "newAccount",
            "isWritable": true,
            "isSigner": true
          }
        ],
        "data": {
          "kind": "structTypeNode",
          "fields": [
            {
              "kind": "structFieldTypeNode",
              "identifier": "discriminator",
              "defaultValueStrategy": "omitted",
              "type": {
                "kind": "integerTypeNode",
                "format": "u32",
                "endian": "le"
              },
              "defaultValue": {
                "kind": "integerValueNode",
                "value": "0"
              },
              "display": {
                "kind": "structFieldDisplayNode",
                "skip": "always"
              }
            },
            {
              "kind": "structFieldTypeNode",
              "identifier": "lamports",
              "type": {
                "kind": "integerTypeNode",
                "format": "u64",
                "endian": "le"
              }
            },
            {
              "kind": "structFieldTypeNode",
              "identifier": "space",
              "type": {
                "kind": "integerTypeNode",
                "format": "u64",
                "endian": "le"
              }
            },
            {
              "kind": "structFieldTypeNode",
              "identifier": "programAddress",
              "type": {
                "kind": "publicKeyTypeNode"
              }
            }
          ]
        },
        "discriminators": [
          {
            "kind": "fieldDiscriminatorNode",
            "path": "discriminator",
            "offset": 0
          }
        ]
      },
      {
        "kind": "instructionNode",
        "identifier": "assign",
        "accounts": [
          {
            "kind": "instructionAccountNode",
            "identifier": "account",
            "isWritable": true,
            "isSigner": true
          }
        ],
        "data": {
          "kind": "structTypeNode",
          "fields": [
            {
              "kind": "structFieldTypeNode",
              "identifier": "discriminator",
              "defaultValueStrategy": "omitted",
              "type": {
                "kind": "integerTypeNode",
                "format": "u32",
                "endian": "le"
              },
              "defaultValue": {
                "kind": "integerValueNode",
                "value": "1"
              },
              "display": {
                "kind": "structFieldDisplayNode",
                "skip": "always"
              }
            },
            {
              "kind": "structFieldTypeNode",
              "identifier": "programAddress",
              "type": {
                "kind": "publicKeyTypeNode"
              }
            }
          ]
        },
        "discriminators": [
          {
            "kind": "fieldDiscriminatorNode",
            "path": "discriminator",
            "offset": 0
          }
        ]
      },
      {
        "kind": "instructionNode",
        "identifier": "transferSol",
        "accounts": [
          {
            "kind": "instructionAccountNode",
            "identifier": "source",
            "isWritable": true,
            "isSigner": true,
            "display": {
              "kind": "instructionAccountDisplayNode",
              "label": "From"
            }
          },
          {
            "kind": "instructionAccountNode",
            "identifier": "destination",
            "isWritable": true,
            "isSigner": false,
            "display": {
              "kind": "instructionAccountDisplayNode",
              "label": "To"
            }
          }
        ],
        "data": {
          "kind": "structTypeNode",
          "fields": [
            {
              "kind": "structFieldTypeNode",
              "identifier": "discriminator",
              "defaultValueStrategy": "omitted",
              "type": {
                "kind": "integerTypeNode",
                "format": "u32",
                "endian": "le"
              },
              "defaultValue": {
                "kind": "integerValueNode",
                "value": "2"
              },
              "display": {
                "kind": "structFieldDisplayNode",
                "skip": "always"
              }
            },
            {
              "kind": "structFieldTypeNode",
              "identifier": "amount",
              "type": {
                "kind": "integerTypeNode",
                "format": "u64",
                "endian": "le",
                "display": {
                  "kind": "amountNumberDisplayNode",
                  "decimals": {
                    "kind": "integerValueNode",
                    "value": "9"
                  },
                  "unit": {
                    "kind": "stringValueNode",
                    "string": "SOL"
                  }
                }
              },
              "display": {
                "kind": "structFieldDisplayNode",
                "label": "Amount"
              }
            }
          ]
        },
        "discriminators": [
          {
            "kind": "fieldDiscriminatorNode",
            "path": "discriminator",
            "offset": 0
          }
        ],
        "display": {
          "kind": "instructionDisplayNode",
          "intent": "Transfer SOL",
          "interpolatedIntent": "Transfer ${data.amount} from ${accounts.source} to ${accounts.destination}"
        }
      },
      {
        "kind": "instructionNode",
        "identifier": "createAccountWithSeed",
        "accounts": [
          {
            "kind": "instructionAccountNode",
            "identifier": "payer",
            "isWritable": true,
            "isSigner": true
          },
          {
            "kind": "instructionAccountNode",
            "identifier": "newAccount",
            "isWritable": true,
            "isSigner": false
          },
          {
            "kind": "instructionAccountNode",
            "identifier": "baseAccount",
            "isWritable": false,
            "isSigner": true
          }
        ],
        "data": {
          "kind": "structTypeNode",
          "fields": [
            {
              "kind": "structFieldTypeNode",
              "identifier": "discriminator",
              "defaultValueStrategy": "omitted",
              "type": {
                "kind": "integerTypeNode",
                "format": "u32",
                "endian": "le"
              },
              "defaultValue": {
                "kind": "integerValueNode",
                "value": "3"
              },
              "display": {
                "kind": "structFieldDisplayNode",
                "skip": "always"
              }
            },
            {
              "kind": "structFieldTypeNode",
              "identifier": "base",
              "type": {
                "kind": "publicKeyTypeNode"
              }
            },
            {
              "kind": "structFieldTypeNode",
              "identifier": "seed",
              "type": {
                "kind": "stringTypeNode",
                "encoding": "utf8",
                "transforms": [
                  {
                    "kind": "sizePrefixTransformNode",
                    "prefix": {
                      "kind": "integerTypeNode",
                      "format": "u64",
                      "endian": "le"
                    }
                  }
                ]
              }
            },
            {
              "kind": "structFieldTypeNode",
              "identifier": "amount",
              "type": {
                "kind": "integerTypeNode",
                "format": "u64",
                "endian": "le"
              }
            },
            {
              "kind": "structFieldTypeNode",
              "identifier": "space",
              "type": {
                "kind": "integerTypeNode",
                "format": "u64",
                "endian": "le"
              }
            },
            {
              "kind": "structFieldTypeNode",
              "identifier": "programAddress",
              "type": {
                "kind": "publicKeyTypeNode"
              }
            }
          ]
        },
        "discriminators": [
          {
            "kind": "fieldDiscriminatorNode",
            "path": "discriminator",
            "offset": 0
          }
        ]
      },
      {
        "kind": "instructionNode",
        "identifier": "advanceNonceAccount",
        "accounts": [
          {
            "kind": "instructionAccountNode",
            "identifier": "nonceAccount",
            "isWritable": true,
            "isSigner": false
          },
          {
            "kind": "instructionAccountNode",
            "identifier": "recentBlockhashesSysvar",
            "isWritable": false,
            "isSigner": false,
            "defaultValue": {
              "kind": "publicKeyValueNode",
              "publicKey": "SysvarRecentB1ockHashes11111111111111111111"
            }
          },
          {
            "kind": "instructionAccountNode",
            "identifier": "nonceAuthority",
            "isWritable": false,
            "isSigner": true
          }
        ],
        "data": {
          "kind": "structTypeNode",
          "fields": [
            {
              "kind": "structFieldTypeNode",
              "identifier": "discriminator",
              "defaultValueStrategy": "omitted",
              "type": {
                "kind": "integerTypeNode",
                "format": "u32",
                "endian": "le"
              },
              "defaultValue": {
                "kind": "integerValueNode",
                "value": "4"
              },
              "display": {
                "kind": "structFieldDisplayNode",
                "skip": "always"
              }
            }
          ]
        },
        "discriminators": [
          {
            "kind": "fieldDiscriminatorNode",
            "path": "discriminator",
            "offset": 0
          }
        ]
      },
      {
        "kind": "instructionNode",
        "identifier": "withdrawNonceAccount",
        "accounts": [
          {
            "kind": "instructionAccountNode",
            "identifier": "nonceAccount",
            "isWritable": true,
            "isSigner": false
          },
          {
            "kind": "instructionAccountNode",
            "identifier": "recipientAccount",
            "isWritable": true,
            "isSigner": false
          },
          {
            "kind": "instructionAccountNode",
            "identifier": "recentBlockhashesSysvar",
            "isWritable": false,
            "isSigner": false,
            "defaultValue": {
              "kind": "publicKeyValueNode",
              "publicKey": "SysvarRecentB1ockHashes11111111111111111111"
            }
          },
          {
            "kind": "instructionAccountNode",
            "identifier": "rentSysvar",
            "isWritable": false,
            "isSigner": false,
            "defaultValue": {
              "kind": "publicKeyValueNode",
              "publicKey": "SysvarRent111111111111111111111111111111111"
            }
          },
          {
            "kind": "instructionAccountNode",
            "identifier": "nonceAuthority",
            "isWritable": false,
            "isSigner": true
          }
        ],
        "data": {
          "kind": "structTypeNode",
          "fields": [
            {
              "kind": "structFieldTypeNode",
              "identifier": "discriminator",
              "defaultValueStrategy": "omitted",
              "type": {
                "kind": "integerTypeNode",
                "format": "u32",
                "endian": "le"
              },
              "defaultValue": {
                "kind": "integerValueNode",
                "value": "5"
              },
              "display": {
                "kind": "structFieldDisplayNode",
                "skip": "always"
              }
            },
            {
              "kind": "structFieldTypeNode",
              "identifier": "withdrawAmount",
              "type": {
                "kind": "integerTypeNode",
                "format": "u64",
                "endian": "le"
              }
            }
          ]
        },
        "discriminators": [
          {
            "kind": "fieldDiscriminatorNode",
            "path": "discriminator",
            "offset": 0
          }
        ]
      },
      {
        "kind": "instructionNode",
        "identifier": "initializeNonceAccount",
        "accounts": [
          {
            "kind": "instructionAccountNode",
            "identifier": "nonceAccount",
            "isWritable": true,
            "isSigner": false
          },
          {
            "kind": "instructionAccountNode",
            "identifier": "recentBlockhashesSysvar",
            "isWritable": false,
            "isSigner": false,
            "defaultValue": {
              "kind": "publicKeyValueNode",
              "publicKey": "SysvarRecentB1ockHashes11111111111111111111"
            }
          },
          {
            "kind": "instructionAccountNode",
            "identifier": "rentSysvar",
            "isWritable": false,
            "isSigner": false,
            "defaultValue": {
              "kind": "publicKeyValueNode",
              "publicKey": "SysvarRent111111111111111111111111111111111"
            }
          }
        ],
        "data": {
          "kind": "structTypeNode",
          "fields": [
            {
              "kind": "structFieldTypeNode",
              "identifier": "discriminator",
              "defaultValueStrategy": "omitted",
              "type": {
                "kind": "integerTypeNode",
                "format": "u32",
                "endian": "le"
              },
              "defaultValue": {
                "kind": "integerValueNode",
                "value": "6"
              },
              "display": {
                "kind": "structFieldDisplayNode",
                "skip": "always"
              }
            },
            {
              "kind": "structFieldTypeNode",
              "identifier": "nonceAuthority",
              "type": {
                "kind": "publicKeyTypeNode"
              }
            }
          ]
        },
        "discriminators": [
          {
            "kind": "fieldDiscriminatorNode",
            "path": "discriminator",
            "offset": 0
          }
        ]
      },
      {
        "kind": "instructionNode",
        "identifier": "authorizeNonceAccount",
        "accounts": [
          {
            "kind": "instructionAccountNode",
            "identifier": "nonceAccount",
            "isWritable": true,
            "isSigner": false
          },
          {
            "kind": "instructionAccountNode",
            "identifier": "nonceAuthority",
            "isWritable": false,
            "isSigner": true
          }
        ],
        "data": {
          "kind": "structTypeNode",
          "fields": [
            {
              "kind": "structFieldTypeNode",
              "identifier": "discriminator",
              "defaultValueStrategy": "omitted",
              "type": {
                "kind": "integerTypeNode",
                "format": "u32",
                "endian": "le"
              },
              "defaultValue": {
                "kind": "integerValueNode",
                "value": "7"
              },
              "display": {
                "kind": "structFieldDisplayNode",
                "skip": "always"
              }
            },
            {
              "kind": "structFieldTypeNode",
              "identifier": "newNonceAuthority",
              "type": {
                "kind": "publicKeyTypeNode"
              }
            }
          ]
        },
        "discriminators": [
          {
            "kind": "fieldDiscriminatorNode",
            "path": "discriminator",
            "offset": 0
          }
        ]
      },
      {
        "kind": "instructionNode",
        "identifier": "allocate",
        "accounts": [
          {
            "kind": "instructionAccountNode",
            "identifier": "newAccount",
            "isWritable": true,
            "isSigner": true
          }
        ],
        "data": {
          "kind": "structTypeNode",
          "fields": [
            {
              "kind": "structFieldTypeNode",
              "identifier": "discriminator",
              "defaultValueStrategy": "omitted",
              "type": {
                "kind": "integerTypeNode",
                "format": "u32",
                "endian": "le"
              },
              "defaultValue": {
                "kind": "integerValueNode",
                "value": "8"
              },
              "display": {
                "kind": "structFieldDisplayNode",
                "skip": "always"
              }
            },
            {
              "kind": "structFieldTypeNode",
              "identifier": "space",
              "type": {
                "kind": "integerTypeNode",
                "format": "u64",
                "endian": "le"
              }
            }
          ]
        },
        "discriminators": [
          {
            "kind": "fieldDiscriminatorNode",
            "path": "discriminator",
            "offset": 0
          }
        ]
      },
      {
        "kind": "instructionNode",
        "identifier": "allocateWithSeed",
        "accounts": [
          {
            "kind": "instructionAccountNode",
            "identifier": "newAccount",
            "isWritable": true,
            "isSigner": false
          },
          {
            "kind": "instructionAccountNode",
            "identifier": "baseAccount",
            "isWritable": false,
            "isSigner": true
          }
        ],
        "data": {
          "kind": "structTypeNode",
          "fields": [
            {
              "kind": "structFieldTypeNode",
              "identifier": "discriminator",
              "defaultValueStrategy": "omitted",
              "type": {
                "kind": "integerTypeNode",
                "format": "u32",
                "endian": "le"
              },
              "defaultValue": {
                "kind": "integerValueNode",
                "value": "9"
              },
              "display": {
                "kind": "structFieldDisplayNode",
                "skip": "always"
              }
            },
            {
              "kind": "structFieldTypeNode",
              "identifier": "base",
              "type": {
                "kind": "publicKeyTypeNode"
              }
            },
            {
              "kind": "structFieldTypeNode",
              "identifier": "seed",
              "type": {
                "kind": "stringTypeNode",
                "encoding": "utf8",
                "transforms": [
                  {
                    "kind": "sizePrefixTransformNode",
                    "prefix": {
                      "kind": "integerTypeNode",
                      "format": "u64",
                      "endian": "le"
                    }
                  }
                ]
              }
            },
            {
              "kind": "structFieldTypeNode",
              "identifier": "space",
              "type": {
                "kind": "integerTypeNode",
                "format": "u64",
                "endian": "le"
              }
            },
            {
              "kind": "structFieldTypeNode",
              "identifier": "programAddress",
              "type": {
                "kind": "publicKeyTypeNode"
              }
            }
          ]
        },
        "discriminators": [
          {
            "kind": "fieldDiscriminatorNode",
            "path": "discriminator",
            "offset": 0
          }
        ]
      },
      {
        "kind": "instructionNode",
        "identifier": "assignWithSeed",
        "accounts": [
          {
            "kind": "instructionAccountNode",
            "identifier": "account",
            "isWritable": true,
            "isSigner": false
          },
          {
            "kind": "instructionAccountNode",
            "identifier": "baseAccount",
            "isWritable": false,
            "isSigner": true
          }
        ],
        "data": {
          "kind": "structTypeNode",
          "fields": [
            {
              "kind": "structFieldTypeNode",
              "identifier": "discriminator",
              "defaultValueStrategy": "omitted",
              "type": {
                "kind": "integerTypeNode",
                "format": "u32",
                "endian": "le"
              },
              "defaultValue": {
                "kind": "integerValueNode",
                "value": "10"
              },
              "display": {
                "kind": "structFieldDisplayNode",
                "skip": "always"
              }
            },
            {
              "kind": "structFieldTypeNode",
              "identifier": "base",
              "type": {
                "kind": "publicKeyTypeNode"
              }
            },
            {
              "kind": "structFieldTypeNode",
              "identifier": "seed",
              "type": {
                "kind": "stringTypeNode",
                "encoding": "utf8",
                "transforms": [
                  {
                    "kind": "sizePrefixTransformNode",
                    "prefix": {
                      "kind": "integerTypeNode",
                      "format": "u64",
                      "endian": "le"
                    }
                  }
                ]
              }
            },
            {
              "kind": "structFieldTypeNode",
              "identifier": "programAddress",
              "type": {
                "kind": "publicKeyTypeNode"
              }
            }
          ]
        },
        "discriminators": [
          {
            "kind": "fieldDiscriminatorNode",
            "path": "discriminator",
            "offset": 0
          }
        ]
      },
      {
        "kind": "instructionNode",
        "identifier": "transferSolWithSeed",
        "accounts": [
          {
            "kind": "instructionAccountNode",
            "identifier": "source",
            "isWritable": true,
            "isSigner": false
          },
          {
            "kind": "instructionAccountNode",
            "identifier": "baseAccount",
            "isWritable": false,
            "isSigner": true
          },
          {
            "kind": "instructionAccountNode",
            "identifier": "destination",
            "isWritable": true,
            "isSigner": false
          }
        ],
        "data": {
          "kind": "structTypeNode",
          "fields": [
            {
              "kind": "structFieldTypeNode",
              "identifier": "discriminator",
              "defaultValueStrategy": "omitted",
              "type": {
                "kind": "integerTypeNode",
                "format": "u32",
                "endian": "le"
              },
              "defaultValue": {
                "kind": "integerValueNode",
                "value": "11"
              },
              "display": {
                "kind": "structFieldDisplayNode",
                "skip": "always"
              }
            },
            {
              "kind": "structFieldTypeNode",
              "identifier": "amount",
              "type": {
                "kind": "integerTypeNode",
                "format": "u64",
                "endian": "le"
              }
            },
            {
              "kind": "structFieldTypeNode",
              "identifier": "fromSeed",
              "type": {
                "kind": "stringTypeNode",
                "encoding": "utf8",
                "transforms": [
                  {
                    "kind": "sizePrefixTransformNode",
                    "prefix": {
                      "kind": "integerTypeNode",
                      "format": "u64",
                      "endian": "le"
                    }
                  }
                ]
              }
            },
            {
              "kind": "structFieldTypeNode",
              "identifier": "fromOwner",
              "type": {
                "kind": "publicKeyTypeNode"
              }
            }
          ]
        },
        "discriminators": [
          {
            "kind": "fieldDiscriminatorNode",
            "path": "discriminator",
            "offset": 0
          }
        ]
      },
      {
        "kind": "instructionNode",
        "identifier": "upgradeNonceAccount",
        "accounts": [
          {
            "kind": "instructionAccountNode",
            "identifier": "nonceAccount",
            "isWritable": true,
            "isSigner": false
          }
        ],
        "data": {
          "kind": "structTypeNode",
          "fields": [
            {
              "kind": "structFieldTypeNode",
              "identifier": "discriminator",
              "defaultValueStrategy": "omitted",
              "type": {
                "kind": "integerTypeNode",
                "format": "u32",
                "endian": "le"
              },
              "defaultValue": {
                "kind": "integerValueNode",
                "value": "12"
              },
              "display": {
                "kind": "structFieldDisplayNode",
                "skip": "always"
              }
            }
          ]
        },
        "discriminators": [
          {
            "kind": "fieldDiscriminatorNode",
            "path": "discriminator",
            "offset": 0
          }
        ]
      }
    ],
    "definedTypes": [
      {
        "kind": "definedTypeNode",
        "identifier": "nonceVersion",
        "type": {
          "kind": "enumTypeNode",
          "variants": [
            {
              "kind": "enumVariantTypeNode",
              "identifier": "legacy"
            },
            {
              "kind": "enumVariantTypeNode",
              "identifier": "current"
            }
          ],
          "size": {
            "kind": "integerTypeNode",
            "format": "u32",
            "endian": "le"
          }
        }
      },
      {
        "kind": "definedTypeNode",
        "identifier": "nonceState",
        "type": {
          "kind": "enumTypeNode",
          "variants": [
            {
              "kind": "enumVariantTypeNode",
              "identifier": "uninitialized"
            },
            {
              "kind": "enumVariantTypeNode",
              "identifier": "initialized"
            }
          ],
          "size": {
            "kind": "integerTypeNode",
            "format": "u32",
            "endian": "le"
          }
        }
      }
    ],
    "errors": [
      {
        "kind": "errorNode",
        "identifier": "accountAlreadyInUse",
        "code": 0,
        "message": "an account with the same address already exists"
      },
      {
        "kind": "errorNode",
        "identifier": "resultWithNegativeLamports",
        "code": 1,
        "message": "account does not have enough SOL to perform the operation"
      },
      {
        "kind": "errorNode",
        "identifier": "invalidProgramId",
        "code": 2,
        "message": "cannot assign account to this program id"
      },
      {
        "kind": "errorNode",
        "identifier": "invalidAccountDataLength",
        "code": 3,
        "message": "cannot allocate account data of this length"
      },
      {
        "kind": "errorNode",
        "identifier": "maxSeedLengthExceeded",
        "code": 4,
        "message": "length of requested seed is too long"
      },
      {
        "kind": "errorNode",
        "identifier": "addressWithSeedMismatch",
        "code": 5,
        "message": "provided address does not match addressed derived from seed"
      },
      {
        "kind": "errorNode",
        "identifier": "nonceNoRecentBlockhashes",
        "code": 6,
        "message": "advancing stored nonce requires a populated RecentBlockhashes sysvar"
      },
      {
        "kind": "errorNode",
        "identifier": "nonceBlockhashNotExpired",
        "code": 7,
        "message": "stored nonce is still in recent_blockhashes"
      },
      {
        "kind": "errorNode",
        "identifier": "nonceUnexpectedBlockhashValue",
        "code": 8,
        "message": "specified nonce does not match stored nonce"
      }
    ]
  }
}"#
    );
}
