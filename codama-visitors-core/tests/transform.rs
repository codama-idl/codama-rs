use codama_nodes::{
    AccountNode, ArrayTypeNode, Endianness, InstructionArgumentNode, NumberTypeNode, ProgramNode,
    RootNode, StructFieldTypeNode, StructTypeNode, U32, U64, U8,
};
use codama_visitors_core::{fold_struct_type, TransformVisitor};
use pretty_assertions::assert_eq;

/// Builds a small but representative IDL:
/// root -> program -> { account(data: struct{ counter: u32, blob: [u8; 4] }),
///                      instruction(arg: amount: u64) }
fn sample_root() -> RootNode {
    let data = StructTypeNode::new(vec![
        StructFieldTypeNode::new("counter", NumberTypeNode::le(U32)),
        StructFieldTypeNode::new("blob", ArrayTypeNode::fixed(NumberTypeNode::le(U8), 4)),
    ]);
    let account = AccountNode::new("myAccount", data);

    let mut program = ProgramNode::new("myProgram", "Myprogram1111111111111111111111111111111111");
    program.accounts.push(account);

    let arg = InstructionArgumentNode::new("amount", NumberTypeNode::le(U64));
    // InstructionArgumentNode is reachable as a child; attach it via an
    // instruction so the visitor must recurse through the instruction subtree.
    let mut instruction = codama_nodes::InstructionNode {
        name: "myIx".into(),
        ..Default::default()
    };
    instruction.arguments.push(arg);
    program.instructions.push(instruction);

    RootNode::new(program)
}

/// The default implementation rebuilds the tree unchanged.
#[test]
fn identity_returns_an_equivalent_tree() {
    struct Identity;
    impl TransformVisitor for Identity {}

    let root = sample_root();
    let out = Identity.visit_root(root.clone());

    assert_eq!(
        serde_json::to_value(&out).unwrap(),
        serde_json::to_value(&root).unwrap(),
    );
}

/// Overriding a single leaf method rewrites every matching node, however deeply
/// nested, while the rest of the tree is preserved.
#[test]
fn overriding_a_leaf_rewrites_every_matching_node() {
    struct ForceBigEndian;
    impl TransformVisitor for ForceBigEndian {
        fn visit_number_type(&mut self, mut node: NumberTypeNode) -> NumberTypeNode {
            node.endian = Endianness::Be;
            node
        }
    }

    let out = ForceBigEndian.visit_root(sample_root());
    let json = serde_json::to_string(&out).unwrap();

    // Every number (struct field, array item, instruction argument) flipped.
    assert!(
        !json.contains(r#""endian":"le""#),
        "no little-endian numbers should remain: {json}"
    );
    assert!(json.contains(r#""endian":"be""#), "{json}");
}

/// Overriding a container method and calling the matching `fold_*` recurses into
/// children (the equivalent of the TS `next` hook), so accumulated state sees
/// the whole subtree.
#[test]
fn fold_from_an_override_still_recurses() {
    #[derive(Default)]
    struct Counter {
        numbers: usize,
        structs: usize,
    }
    impl TransformVisitor for Counter {
        fn visit_number_type(&mut self, node: NumberTypeNode) -> NumberTypeNode {
            self.numbers += 1;
            node
        }
        fn visit_struct_type(&mut self, node: StructTypeNode) -> StructTypeNode {
            self.structs += 1;
            fold_struct_type(self, node) // recurse into the struct's fields
        }
    }

    let mut counter = Counter::default();
    counter.visit_root(sample_root());

    // 1 struct (the account data).
    assert_eq!(counter.structs, 1);
    // 3 numbers: struct field `counter` (u32), array item (u8), instruction
    // argument `amount` (u64).
    assert_eq!(counter.numbers, 3);
}

/// A fixture whose six little-endian numbers are reachable only through the
/// trickier child paths: a `sizePrefix` wrapper's prefix, an `option`'s item and
/// prefix, an `amount`'s nested number, and an `enum`'s nested size. Rewriting
/// every number and asserting none stay little-endian proves those folds recurse
/// (an identity check cannot — not recursing still returns an equal node).
const RICH_IDL: &str = r#"{
  "kind": "rootNode",
  "standard": "codama",
  "version": "1.0.0",
  "program": {
    "kind": "programNode",
    "name": "myProgram",
    "publicKey": "Myprogram1111111111111111111111111111111111",
    "version": "0.0.0",
    "accounts": [{
      "kind": "accountNode",
      "name": "myAccount",
      "data": { "kind": "structTypeNode", "fields": [
        { "kind": "structFieldTypeNode", "name": "a",
          "type": { "kind": "numberTypeNode", "format": "u32", "endian": "le" } },
        { "kind": "structFieldTypeNode", "name": "b", "type": {
          "kind": "sizePrefixTypeNode",
          "type": { "kind": "stringTypeNode", "encoding": "utf8" },
          "prefix": { "kind": "numberTypeNode", "format": "u32", "endian": "le" } } },
        { "kind": "structFieldTypeNode", "name": "c", "type": {
          "kind": "optionTypeNode",
          "item": { "kind": "numberTypeNode", "format": "u8", "endian": "le" },
          "prefix": { "kind": "numberTypeNode", "format": "u8", "endian": "le" } } },
        { "kind": "structFieldTypeNode", "name": "d", "type": {
          "kind": "amountTypeNode", "decimals": 9,
          "number": { "kind": "numberTypeNode", "format": "u64", "endian": "le" } } }
      ] }
    }],
    "instructions": [],
    "definedTypes": [{
      "kind": "definedTypeNode", "name": "myEnum", "type": {
        "kind": "enumTypeNode",
        "size": { "kind": "numberTypeNode", "format": "u32", "endian": "le" },
        "variants": [{ "kind": "enumEmptyVariantTypeNode", "name": "x" }] } }],
    "pdas": [],
    "events": [],
    "errors": [],
    "constants": []
  },
  "additionalPrograms": []
}"#;

#[test]
fn rich_identity_round_trips() {
    struct Identity;
    impl TransformVisitor for Identity {}

    let root: RootNode = serde_json::from_str(RICH_IDL).unwrap();
    let out = Identity.visit_root(root.clone());

    assert_eq!(
        serde_json::to_value(&out).unwrap(),
        serde_json::to_value(&root).unwrap(),
    );
}

#[test]
fn rewrite_reaches_numbers_under_every_child_path() {
    struct ForceBigEndian;
    impl TransformVisitor for ForceBigEndian {
        fn visit_number_type(&mut self, mut node: NumberTypeNode) -> NumberTypeNode {
            node.endian = Endianness::Be;
            node
        }
    }

    let root: RootNode = serde_json::from_str(RICH_IDL).unwrap();
    let out = ForceBigEndian.visit_root(root);
    let json = serde_json::to_string(&out).unwrap();

    // Every number — including those under sizePrefix.prefix, option.item/prefix,
    // amount.number and enum.size — must have been visited.
    assert!(
        !json.contains(r#""endian":"le""#),
        "a number was not reached by the traversal: {json}"
    );
    assert_eq!(json.matches(r#""endian":"be""#).count(), 6);
}
