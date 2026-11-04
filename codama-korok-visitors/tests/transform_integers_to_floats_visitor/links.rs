use super::utils::{
    account_with_field, float, integer, root_with_programs, transform_integers_to_floats,
    type_link_with_program,
};
use codama_nodes::test_utils::id;
use codama_nodes::{
    AccountNode, DefinedTypeLinkNode, DefinedTypeNode, FloatFormat::F64, FloatTypeNode,
    FloatValueNode, IntegerFormat::U64, IntegerTypeNode, ProgramNode, StructFieldTypeNode,
    StructTypeNode, TypeNode,
};

fn main_program() -> ProgramNode {
    ProgramNode::new(id("main"), "Main111111111111111111111111111111111111111")
        .add_defined_type(DefinedTypeNode::new(id("rate"), FloatTypeNode::le(F64)))
        .add_defined_type(DefinedTypeNode::new(
            id("feeRate"),
            DefinedTypeLinkNode::new(id("rate")),
        ))
        .add_defined_type(DefinedTypeNode::new(id("count"), IntegerTypeNode::le(U64)))
        .add_defined_type(DefinedTypeNode::new(
            id("selfLoop"),
            DefinedTypeLinkNode::new(id("selfLoop")),
        ))
        .add_defined_type(DefinedTypeNode::new(
            id("loopA"),
            DefinedTypeLinkNode::new(id("loopB")),
        ))
        .add_defined_type(DefinedTypeNode::new(
            id("loopB"),
            DefinedTypeLinkNode::new(id("loopA")),
        ))
        .add_defined_type(DefinedTypeNode::new(
            id("intoLoop"),
            DefinedTypeLinkNode::new(id("loopA")),
        ))
}

fn other_program() -> ProgramNode {
    ProgramNode::new(id("other"), "Other11111111111111111111111111111111111111")
        .add_defined_type(DefinedTypeNode::new(id("rate"), IntegerTypeNode::le(U64)))
        .add_defined_type(DefinedTypeNode::new(id("price"), FloatTypeNode::le(F64)))
        .add_defined_type(DefinedTypeNode::new(
            id("spotPrice"),
            DefinedTypeLinkNode::new(id("price")),
        ))
        .add_defined_type(DefinedTypeNode::new(
            id("mainRate"),
            type_link_with_program("rate", "main"),
        ))
}

#[test]
fn linked_defaults_become_floats() {
    for r#type in [
        TypeNode::from(DefinedTypeLinkNode::new(id("rate"))),
        DefinedTypeLinkNode::new(id("feeRate")).into(),
        type_link_with_program("rate", "main").into(),
        type_link_with_program("price", "other").into(),
        type_link_with_program("spotPrice", "other").into(),
        type_link_with_program("mainRate", "other").into(),
    ] {
        let root = |value| {
            root_with_programs(
                main_program().add_account(account_with_field(r#type.clone(), value)),
                other_program(),
            )
        };
        assert_eq!(transform_integers_to_floats(root(integer())), root(float()));
    }
}

#[test]
fn cycles_keep_the_integer() {
    for identifier in ["selfLoop", "loopA", "intoLoop"] {
        let account = account_with_field(DefinedTypeLinkNode::new(id(identifier)), integer());
        let root = root_with_programs(main_program().add_account(account), other_program());
        assert_eq!(transform_integers_to_floats(root.clone()), root);
    }
}

#[test]
fn dangling_links_keep_the_integer() {
    for r#type in [
        DefinedTypeLinkNode::new(id("missing")),
        DefinedTypeLinkNode::new(id("price")),
        type_link_with_program("rate", "unknown"),
        type_link_with_program("missing", "other"),
    ] {
        let account = account_with_field(r#type, integer());
        let root = root_with_programs(main_program().add_account(account), other_program());
        assert_eq!(transform_integers_to_floats(root.clone()), root);
    }
}

#[test]
fn links_to_integer_types_keep_the_value() {
    for (r#type, value) in [
        (DefinedTypeLinkNode::new(id("count")), integer()),
        (type_link_with_program("rate", "other"), integer()),
        (
            DefinedTypeLinkNode::new(id("count")),
            FloatValueNode::new(2.0).into(),
        ),
    ] {
        let account = account_with_field(r#type, value);
        let root = root_with_programs(main_program().add_account(account), other_program());
        assert_eq!(transform_integers_to_floats(root.clone()), root);
    }
}

#[test]
fn links_without_program_resolve_in_the_program_of_the_position() {
    let account = |rate, price| {
        let field = |identifier: &str, value| StructFieldTypeNode {
            default_value: Box::new(Some(value)),
            ..StructFieldTypeNode::new(id(identifier), DefinedTypeLinkNode::new(id(identifier)))
        };
        AccountNode::new(
            id("quote"),
            StructTypeNode::new(vec![field("rate", rate), field("price", price)]),
        )
    };
    assert_eq!(
        transform_integers_to_floats(root_with_programs(
            main_program().add_account(account(integer(), integer())),
            other_program().add_account(account(integer(), integer())),
        )),
        root_with_programs(
            main_program().add_account(account(float(), integer())),
            other_program().add_account(account(integer(), float())),
        )
    );
}
