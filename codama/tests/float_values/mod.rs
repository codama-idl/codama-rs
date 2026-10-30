use crate::get_path;
use codama::{Codama, NodeTrait};

#[test]
fn get_idl() {
    let codama = Codama::load(get_path("float_values/crate")).unwrap();
    let idl = codama.get_idl().unwrap().to_json_pretty().unwrap();

    assert_eq!(
        idl,
        r#"{
  "kind": "rootNode",
  "standard": "codama",
  "version": "2.0.0",
  "program": {
    "kind": "programNode",
    "identifier": "exchange",
    "publicKey": "Exchange11111111111111111111111111111111111",
    "version": "1.0.0",
    "accounts": [
      {
        "kind": "accountNode",
        "identifier": "config",
        "data": {
          "kind": "structTypeNode",
          "fields": [
            {
              "kind": "structFieldTypeNode",
              "identifier": "bonus",
              "type": {
                "kind": "floatTypeNode",
                "format": "f32",
                "endian": "le"
              },
              "defaultValue": {
                "kind": "floatValueNode",
                "value": "1"
              }
            },
            {
              "kind": "structFieldTypeNode",
              "identifier": "spread",
              "type": {
                "kind": "definedTypeLinkNode",
                "identifier": "rate"
              },
              "defaultValue": {
                "kind": "floatValueNode",
                "value": "1"
              }
            },
            {
              "kind": "structFieldTypeNode",
              "identifier": "scale",
              "type": {
                "kind": "floatTypeNode",
                "format": "f64",
                "endian": "le"
              },
              "defaultValue": {
                "kind": "floatValueNode",
                "value": "1"
              }
            },
            {
              "kind": "structFieldTypeNode",
              "identifier": "factor",
              "defaultValueStrategy": "omitted",
              "type": {
                "kind": "floatTypeNode",
                "format": "f64",
                "endian": "le"
              },
              "defaultValue": {
                "kind": "floatValueNode",
                "value": "1"
              }
            },
            {
              "kind": "structFieldTypeNode",
              "identifier": "weight",
              "type": {
                "kind": "floatTypeNode",
                "format": "f64",
                "endian": "le"
              },
              "defaultValue": {
                "kind": "floatValueNode",
                "value": "1"
              }
            },
            {
              "kind": "structFieldTypeNode",
              "identifier": "rate",
              "type": {
                "kind": "definedTypeLinkNode",
                "identifier": "rate"
              },
              "defaultValue": {
                "kind": "floatValueNode",
                "value": "1"
              }
            },
            {
              "kind": "structFieldTypeNode",
              "identifier": "fee",
              "type": {
                "kind": "definedTypeLinkNode",
                "identifier": "feeRate"
              },
              "defaultValue": {
                "kind": "floatValueNode",
                "value": "1"
              }
            },
            {
              "kind": "structFieldTypeNode",
              "identifier": "maxOrders",
              "type": {
                "kind": "definedTypeLinkNode",
                "identifier": "count"
              },
              "defaultValue": {
                "kind": "integerValueNode",
                "value": "1"
              }
            },
            {
              "kind": "structFieldTypeNode",
              "identifier": "threshold",
              "type": {
                "kind": "optionTypeNode",
                "item": {
                  "kind": "floatTypeNode",
                  "format": "f64",
                  "endian": "le"
                },
                "prefix": {
                  "kind": "integerTypeNode",
                  "format": "u8",
                  "endian": "le"
                }
              },
              "defaultValue": {
                "kind": "integerValueNode",
                "value": "1"
              }
            },
            {
              "kind": "structFieldTypeNode",
              "identifier": "label",
              "type": {
                "kind": "stringTypeNode",
                "encoding": "utf8",
                "transforms": [
                  {
                    "kind": "sentinelTransformNode",
                    "sentinel": {
                      "kind": "constantValueNode",
                      "type": {
                        "kind": "definedTypeLinkNode",
                        "identifier": "rate"
                      },
                      "value": {
                        "kind": "floatValueNode",
                        "value": "1"
                      }
                    }
                  }
                ]
              }
            },
            {
              "kind": "structFieldTypeNode",
              "identifier": "memo",
              "type": {
                "kind": "stringTypeNode",
                "encoding": "utf8",
                "transforms": [
                  {
                    "kind": "hiddenPrefixTransformNode",
                    "prefix": [
                      {
                        "kind": "constantValueNode",
                        "type": {
                          "kind": "floatTypeNode",
                          "format": "f32",
                          "endian": "le"
                        },
                        "value": {
                          "kind": "floatValueNode",
                          "value": "1"
                        }
                      },
                      {
                        "kind": "constantValueNode",
                        "type": {
                          "kind": "floatTypeNode",
                          "format": "f64",
                          "endian": "le"
                        },
                        "value": {
                          "kind": "floatValueNode",
                          "value": "-2"
                        }
                      }
                    ]
                  }
                ]
              }
            },
            {
              "kind": "structFieldTypeNode",
              "identifier": "limit",
              "type": {
                "kind": "floatTypeNode",
                "format": "f64",
                "endian": "le"
              },
              "defaultValue": {
                "kind": "floatValueNode",
                "value": "1"
              }
            },
            {
              "kind": "structFieldTypeNode",
              "identifier": "cap",
              "type": {
                "kind": "definedTypeLinkNode",
                "identifier": "rate"
              },
              "defaultValue": {
                "kind": "floatValueNode",
                "value": "1"
              }
            }
          ]
        }
      }
    ],
    "instructions": [
      {
        "kind": "instructionNode",
        "identifier": "setScale",
        "data": {
          "kind": "structTypeNode",
          "fields": [
            {
              "kind": "structFieldTypeNode",
              "identifier": "discriminator",
              "defaultValueStrategy": "omitted",
              "type": {
                "kind": "integerTypeNode",
                "format": "u8",
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
              "identifier": "arg0",
              "type": {
                "kind": "floatTypeNode",
                "format": "f32",
                "endian": "le"
              },
              "defaultValue": {
                "kind": "floatValueNode",
                "value": "1"
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
        "identifier": "setFee",
        "data": {
          "kind": "structTypeNode",
          "fields": [
            {
              "kind": "structFieldTypeNode",
              "identifier": "discriminator",
              "defaultValueStrategy": "omitted",
              "type": {
                "kind": "integerTypeNode",
                "format": "u8",
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
              "identifier": "arg0",
              "type": {
                "kind": "definedTypeLinkNode",
                "identifier": "rate"
              },
              "defaultValue": {
                "kind": "floatValueNode",
                "value": "1"
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
        "identifier": "updateRate",
        "accounts": [
          {
            "kind": "instructionAccountNode",
            "identifier": "rateAccount",
            "isWritable": true,
            "isSigner": false,
            "defaultValue": {
              "kind": "pdaValueNode",
              "pda": {
                "kind": "pdaLinkNode",
                "identifier": "ratePda"
              },
              "seeds": [
                {
                  "kind": "pdaSeedValueNode",
                  "identifier": "rate",
                  "value": {
                    "kind": "floatValueNode",
                    "value": "1"
                  }
                }
              ]
            }
          },
          {
            "kind": "instructionAccountNode",
            "identifier": "linkedRateAccount",
            "isWritable": true,
            "isSigner": false,
            "defaultValue": {
              "kind": "pdaValueNode",
              "pda": {
                "kind": "pdaLinkNode",
                "identifier": "linkedRatePda"
              },
              "seeds": [
                {
                  "kind": "pdaSeedValueNode",
                  "identifier": "rate",
                  "value": {
                    "kind": "floatValueNode",
                    "value": "1"
                  }
                }
              ]
            }
          },
          {
            "kind": "instructionAccountNode",
            "identifier": "fieldRateAccount",
            "isWritable": true,
            "isSigner": false,
            "defaultValue": {
              "kind": "pdaValueNode",
              "pda": {
                "kind": "pdaLinkNode",
                "identifier": "fieldRatePda"
              },
              "seeds": [
                {
                  "kind": "pdaSeedValueNode",
                  "identifier": "rate",
                  "value": {
                    "kind": "floatValueNode",
                    "value": "1"
                  }
                }
              ]
            }
          },
          {
            "kind": "instructionAccountNode",
            "identifier": "ownRateAccount",
            "isWritable": false,
            "isSigner": false,
            "defaultValue": {
              "kind": "pdaValueNode",
              "pda": {
                "kind": "pdaLinkNode",
                "identifier": "ratePda",
                "program": {
                  "kind": "programLinkNode",
                  "identifier": "exchange"
                }
              },
              "seeds": [
                {
                  "kind": "pdaSeedValueNode",
                  "identifier": "rate",
                  "value": {
                    "kind": "floatValueNode",
                    "value": "1"
                  }
                }
              ]
            }
          },
          {
            "kind": "instructionAccountNode",
            "identifier": "priceAccount",
            "isWritable": false,
            "isSigner": false,
            "defaultValue": {
              "kind": "pdaValueNode",
              "pda": {
                "kind": "pdaLinkNode",
                "identifier": "pricePda",
                "program": {
                  "kind": "programLinkNode",
                  "identifier": "oracle"
                }
              },
              "seeds": [
                {
                  "kind": "pdaSeedValueNode",
                  "identifier": "price",
                  "value": {
                    "kind": "floatValueNode",
                    "value": "1"
                  }
                }
              ]
            }
          },
          {
            "kind": "instructionAccountNode",
            "identifier": "linkedPriceAccount",
            "isWritable": false,
            "isSigner": false,
            "defaultValue": {
              "kind": "pdaValueNode",
              "pda": {
                "kind": "pdaLinkNode",
                "identifier": "linkedPricePda",
                "program": {
                  "kind": "programLinkNode",
                  "identifier": "oracle"
                }
              },
              "seeds": [
                {
                  "kind": "pdaSeedValueNode",
                  "identifier": "price",
                  "value": {
                    "kind": "floatValueNode",
                    "value": "1"
                  }
                }
              ]
            }
          }
        ],
        "data": {
          "kind": "structTypeNode",
          "fields": [
            {
              "kind": "structFieldTypeNode",
              "identifier": "bonus",
              "type": {
                "kind": "definedTypeLinkNode",
                "identifier": "rate"
              },
              "defaultValue": {
                "kind": "floatValueNode",
                "value": "1"
              }
            },
            {
              "kind": "structFieldTypeNode",
              "identifier": "scale",
              "type": {
                "kind": "floatTypeNode",
                "format": "f64",
                "endian": "le"
              },
              "defaultValue": {
                "kind": "floatValueNode",
                "value": "1"
              }
            },
            {
              "kind": "structFieldTypeNode",
              "identifier": "amount",
              "type": {
                "kind": "definedTypeLinkNode",
                "identifier": "rate"
              },
              "defaultValue": {
                "kind": "floatValueNode",
                "value": "1"
              }
            },
            {
              "kind": "structFieldTypeNode",
              "identifier": "derived",
              "type": {
                "kind": "publicKeyTypeNode"
              },
              "defaultValue": {
                "kind": "injectedValueNode",
                "key": "derived"
              }
            }
          ]
        },
        "provides": [
          {
            "kind": "providedNode",
            "identifier": "derived",
            "node": {
              "kind": "pdaValueNode",
              "pda": {
                "kind": "pdaLinkNode",
                "identifier": "ratePda"
              },
              "seeds": [
                {
                  "kind": "pdaSeedValueNode",
                  "identifier": "rate",
                  "value": {
                    "kind": "floatValueNode",
                    "value": "1"
                  }
                }
              ]
            }
          }
        ]
      }
    ],
    "definedTypes": [
      {
        "kind": "definedTypeNode",
        "identifier": "rate",
        "type": {
          "kind": "floatTypeNode",
          "format": "f64",
          "endian": "le"
        }
      },
      {
        "kind": "definedTypeNode",
        "identifier": "feeRate",
        "type": {
          "kind": "definedTypeLinkNode",
          "identifier": "rate"
        }
      },
      {
        "kind": "definedTypeNode",
        "identifier": "count",
        "type": {
          "kind": "integerTypeNode",
          "format": "u64",
          "endian": "le"
        }
      },
      {
        "kind": "definedTypeNode",
        "identifier": "pricing",
        "type": {
          "kind": "enumTypeNode",
          "variants": [
            {
              "kind": "enumVariantTypeNode",
              "identifier": "fixed",
              "data": {
                "kind": "structTypeNode",
                "fields": [
                  {
                    "kind": "structFieldTypeNode",
                    "identifier": "rate",
                    "type": {
                      "kind": "definedTypeLinkNode",
                      "identifier": "rate"
                    },
                    "defaultValue": {
                      "kind": "floatValueNode",
                      "value": "1"
                    }
                  }
                ]
              }
            },
            {
              "kind": "enumVariantTypeNode",
              "identifier": "floating"
            }
          ],
          "size": {
            "kind": "integerTypeNode",
            "format": "u8",
            "endian": "le"
          }
        }
      }
    ],
    "pdas": [
      {
        "kind": "pdaNode",
        "identifier": "ratePda",
        "seeds": [
          {
            "kind": "variablePdaSeedNode",
            "identifier": "rate",
            "type": {
              "kind": "floatTypeNode",
              "format": "f64",
              "endian": "le"
            }
          }
        ]
      },
      {
        "kind": "pdaNode",
        "identifier": "linkedRatePda",
        "seeds": [
          {
            "kind": "variablePdaSeedNode",
            "identifier": "rate",
            "type": {
              "kind": "definedTypeLinkNode",
              "identifier": "rate"
            }
          }
        ]
      },
      {
        "kind": "pdaNode",
        "identifier": "fieldRatePda",
        "seeds": [
          {
            "kind": "variablePdaSeedNode",
            "identifier": "rate",
            "type": {
              "kind": "definedTypeLinkNode",
              "identifier": "rate"
            }
          }
        ]
      },
      {
        "kind": "pdaNode",
        "identifier": "constRatePda",
        "seeds": [
          {
            "kind": "constantPdaSeedNode",
            "type": {
              "kind": "floatTypeNode",
              "format": "f32",
              "endian": "le"
            },
            "value": {
              "kind": "floatValueNode",
              "value": "1"
            }
          },
          {
            "kind": "constantPdaSeedNode",
            "type": {
              "kind": "definedTypeLinkNode",
              "identifier": "rate"
            },
            "value": {
              "kind": "floatValueNode",
              "value": "1"
            }
          }
        ]
      }
    ]
  },
  "additionalPrograms": [
    {
      "kind": "programNode",
      "identifier": "oracle",
      "publicKey": "Price111111111111111111111111111111111111111",
      "version": "",
      "definedTypes": [
        {
          "kind": "definedTypeNode",
          "identifier": "price",
          "type": {
            "kind": "floatTypeNode",
            "format": "f64",
            "endian": "le"
          }
        }
      ],
      "pdas": [
        {
          "kind": "pdaNode",
          "identifier": "pricePda",
          "seeds": [
            {
              "kind": "variablePdaSeedNode",
              "identifier": "price",
              "type": {
                "kind": "floatTypeNode",
                "format": "f64",
                "endian": "le"
              }
            }
          ]
        },
        {
          "kind": "pdaNode",
          "identifier": "linkedPricePda",
          "seeds": [
            {
              "kind": "variablePdaSeedNode",
              "identifier": "price",
              "type": {
                "kind": "definedTypeLinkNode",
                "identifier": "price"
              }
            }
          ]
        }
      ]
    }
  ]
}"#
    );
}
