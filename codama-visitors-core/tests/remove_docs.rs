use codama_nodes::RootNode;
use codama_visitors_core::{RemoveDocs, TransformVisitor};

/// Docs appear on the program, an account, a struct field, a defined type and an
/// error — every level a `docs` field can occur in this tree.
const IDL_WITH_DOCS: &str = r#"{
  "kind": "rootNode",
  "standard": "codama",
  "version": "1.0.0",
  "program": {
    "kind": "programNode",
    "name": "myProgram",
    "publicKey": "Myprogram1111111111111111111111111111111111",
    "version": "0.0.0",
    "docs": ["program docs"],
    "accounts": [{
      "kind": "accountNode",
      "name": "myAccount",
      "docs": ["account docs"],
      "data": { "kind": "structTypeNode", "fields": [
        { "kind": "structFieldTypeNode", "name": "field", "docs": ["field docs"],
          "type": { "kind": "numberTypeNode", "format": "u8", "endian": "le" } }
      ] }
    }],
    "instructions": [],
    "definedTypes": [{
      "kind": "definedTypeNode", "name": "myType", "docs": ["type docs"],
      "type": { "kind": "numberTypeNode", "format": "u8", "endian": "le" } }],
    "pdas": [],
    "events": [],
    "errors": [{ "kind": "errorNode", "name": "myError", "code": 1,
                 "message": "boom", "docs": ["error docs"] }],
    "constants": []
  },
  "additionalPrograms": []
}"#;

#[test]
fn removes_docs_from_every_node_but_keeps_the_rest() {
    let root: RootNode = serde_json::from_str(IDL_WITH_DOCS).unwrap();
    let out = RemoveDocs.visit_root(root);
    let json = serde_json::to_string(&out).unwrap();

    // Every `docs` field (and its contents) is gone.
    assert!(!json.contains("docs"), "docs should be stripped: {json}");
    // The rest of the tree is untouched.
    assert!(json.contains(r#""name":"myProgram""#));
    assert!(json.contains(r#""name":"myAccount""#));
    assert!(json.contains(r#""name":"field""#));
    assert!(json.contains(r#""name":"myError""#));
}

#[test]
fn idl_without_docs_is_unchanged() {
    // Same tree minus all the docs — RemoveDocs must be a no-op here.
    let stripped = IDL_WITH_DOCS
        .replace(r#""docs": ["program docs"],"#, "")
        .replace(r#""docs": ["account docs"],"#, "")
        .replace(r#""docs": ["field docs"],"#, "")
        .replace(r#""docs": ["type docs"],"#, "")
        .replace(r#", "docs": ["error docs"]"#, "");

    let root: RootNode = serde_json::from_str(&stripped).unwrap();
    let out = RemoveDocs.visit_root(root.clone());

    assert_eq!(
        serde_json::to_value(&out).unwrap(),
        serde_json::to_value(&root).unwrap(),
    );
}
