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
                "identifier": "nonce_version"
              }
            },
            {
              "kind": "structFieldTypeNode",
              "identifier": "state",
              "type": {
                "kind": "definedTypeLinkNode",
                "identifier": "nonce_state"
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
              "identifier": "lamports_per_signature",
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
        "identifier": "create_account",
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
            "identifier": "new_account",
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
              "identifier": "program_address",
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
              "identifier": "program_address",
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
        "identifier": "transfer_sol",
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
        "identifier": "create_account_with_seed",
        "accounts": [
          {
            "kind": "instructionAccountNode",
            "identifier": "payer",
            "isWritable": true,
            "isSigner": true
          },
          {
            "kind": "instructionAccountNode",
            "identifier": "new_account",
            "isWritable": true,
            "isSigner": false
          },
          {
            "kind": "instructionAccountNode",
            "identifier": "base_account",
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
              "identifier": "program_address",
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
        "identifier": "advance_nonce_account",
        "accounts": [
          {
            "kind": "instructionAccountNode",
            "identifier": "nonce_account",
            "isWritable": true,
            "isSigner": false
          },
          {
            "kind": "instructionAccountNode",
            "identifier": "recent_blockhashes_sysvar",
            "isWritable": false,
            "isSigner": false,
            "defaultValue": {
              "kind": "publicKeyValueNode",
              "publicKey": "SysvarRecentB1ockHashes11111111111111111111"
            }
          },
          {
            "kind": "instructionAccountNode",
            "identifier": "nonce_authority",
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
        "identifier": "withdraw_nonce_account",
        "accounts": [
          {
            "kind": "instructionAccountNode",
            "identifier": "nonce_account",
            "isWritable": true,
            "isSigner": false
          },
          {
            "kind": "instructionAccountNode",
            "identifier": "recipient_account",
            "isWritable": true,
            "isSigner": false
          },
          {
            "kind": "instructionAccountNode",
            "identifier": "recent_blockhashes_sysvar",
            "isWritable": false,
            "isSigner": false,
            "defaultValue": {
              "kind": "publicKeyValueNode",
              "publicKey": "SysvarRecentB1ockHashes11111111111111111111"
            }
          },
          {
            "kind": "instructionAccountNode",
            "identifier": "rent_sysvar",
            "isWritable": false,
            "isSigner": false,
            "defaultValue": {
              "kind": "publicKeyValueNode",
              "publicKey": "SysvarRent111111111111111111111111111111111"
            }
          },
          {
            "kind": "instructionAccountNode",
            "identifier": "nonce_authority",
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
              "identifier": "withdraw_amount",
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
        "identifier": "initialize_nonce_account",
        "accounts": [
          {
            "kind": "instructionAccountNode",
            "identifier": "nonce_account",
            "isWritable": true,
            "isSigner": false
          },
          {
            "kind": "instructionAccountNode",
            "identifier": "recent_blockhashes_sysvar",
            "isWritable": false,
            "isSigner": false,
            "defaultValue": {
              "kind": "publicKeyValueNode",
              "publicKey": "SysvarRecentB1ockHashes11111111111111111111"
            }
          },
          {
            "kind": "instructionAccountNode",
            "identifier": "rent_sysvar",
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
              "identifier": "nonce_authority",
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
        "identifier": "authorize_nonce_account",
        "accounts": [
          {
            "kind": "instructionAccountNode",
            "identifier": "nonce_account",
            "isWritable": true,
            "isSigner": false
          },
          {
            "kind": "instructionAccountNode",
            "identifier": "nonce_authority",
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
              "identifier": "new_nonce_authority",
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
            "identifier": "new_account",
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
        "identifier": "allocate_with_seed",
        "accounts": [
          {
            "kind": "instructionAccountNode",
            "identifier": "new_account",
            "isWritable": true,
            "isSigner": false
          },
          {
            "kind": "instructionAccountNode",
            "identifier": "base_account",
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
              "identifier": "program_address",
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
        "identifier": "assign_with_seed",
        "accounts": [
          {
            "kind": "instructionAccountNode",
            "identifier": "account",
            "isWritable": true,
            "isSigner": false
          },
          {
            "kind": "instructionAccountNode",
            "identifier": "base_account",
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
              "identifier": "program_address",
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
        "identifier": "transfer_sol_with_seed",
        "accounts": [
          {
            "kind": "instructionAccountNode",
            "identifier": "source",
            "isWritable": true,
            "isSigner": false
          },
          {
            "kind": "instructionAccountNode",
            "identifier": "base_account",
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
              "identifier": "from_seed",
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
              "identifier": "from_owner",
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
        "identifier": "upgrade_nonce_account",
        "accounts": [
          {
            "kind": "instructionAccountNode",
            "identifier": "nonce_account",
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
        "identifier": "nonce_version",
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
        "identifier": "nonce_state",
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
        "identifier": "account_already_in_use",
        "code": 0,
        "message": "an account with the same address already exists"
      },
      {
        "kind": "errorNode",
        "identifier": "result_with_negative_lamports",
        "code": 1,
        "message": "account does not have enough SOL to perform the operation"
      },
      {
        "kind": "errorNode",
        "identifier": "invalid_program_id",
        "code": 2,
        "message": "cannot assign account to this program id"
      },
      {
        "kind": "errorNode",
        "identifier": "invalid_account_data_length",
        "code": 3,
        "message": "cannot allocate account data of this length"
      },
      {
        "kind": "errorNode",
        "identifier": "max_seed_length_exceeded",
        "code": 4,
        "message": "length of requested seed is too long"
      },
      {
        "kind": "errorNode",
        "identifier": "address_with_seed_mismatch",
        "code": 5,
        "message": "provided address does not match addressed derived from seed"
      },
      {
        "kind": "errorNode",
        "identifier": "nonce_no_recent_blockhashes",
        "code": 6,
        "message": "advancing stored nonce requires a populated RecentBlockhashes sysvar"
      },
      {
        "kind": "errorNode",
        "identifier": "nonce_blockhash_not_expired",
        "code": 7,
        "message": "stored nonce is still in recent_blockhashes"
      },
      {
        "kind": "errorNode",
        "identifier": "nonce_unexpected_blockhash_value",
        "code": 8,
        "message": "specified nonce does not match stored nonce"
      }
    ]
  }
}"#
    );
}
