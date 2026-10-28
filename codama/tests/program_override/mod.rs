use crate::get_path;
use codama::{Codama, NodeTrait};

/// A `codama_program!` declaration renames the primary program end-to-end
/// through the default plugin pipeline. The crate is named `my-crate-name`, so
/// without the override the program would be `myCrateName`; the address still
/// resolves from `package.metadata.solana.program-id`.
#[test]
fn it_renames_the_primary_program_from_a_codama_program_macro() {
    let codama = Codama::load(get_path("program_override/crate")).unwrap();
    let idl = codama.get_idl().unwrap().to_json_pretty().unwrap();

    assert_eq!(
        idl,
        r#"{
  "kind": "rootNode",
  "standard": "codama",
  "version": "2.0.0",
  "program": {
    "kind": "programNode",
    "identifier": "myOverriddenProgramName",
    "publicKey": "MyProgramAddress1111111111111111111111111",
    "version": "1.2.3",
    "instructions": [
      {
        "kind": "instructionNode",
        "identifier": "create",
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
    ]
  }
}"#
    );
}
