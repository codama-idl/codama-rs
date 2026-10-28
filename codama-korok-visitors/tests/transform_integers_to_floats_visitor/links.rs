use super::utils::{
    account_with_field, float, integer, root_with_programs, transform_integers_to_floats,
    type_link_with_program,
};
use codama_nodes::{
    AccountNode, DefinedTypeLinkNode, DefinedTypeNode, FloatFormat::F64, FloatTypeNode,
    FloatValueNode, IntegerFormat::U64, IntegerTypeNode, ProgramNode, StructFieldTypeNode,
    StructTypeNode, TypeNode,
};

fn main_program() -> ProgramNode {
    ProgramNode::new("main", "Main111111111111111111111111111111111111111")
        .add_defined_type(DefinedTypeNode::new("rate", FloatTypeNode::le(F64)))
        .add_defined_type(DefinedTypeNode::new(
            "feeRate",
            DefinedTypeLinkNode::new("rate"),
        ))
        .add_defined_type(DefinedTypeNode::new("count", IntegerTypeNode::le(U64)))
        .add_defined_type(DefinedTypeNode::new(
            "selfLoop",
            DefinedTypeLinkNode::new("selfLoop"),
        ))
        .add_defined_type(DefinedTypeNode::new(
            "loopA",
            DefinedTypeLinkNode::new("loopB"),
        ))
        .add_defined_type(DefinedTypeNode::new(
            "loopB",
            DefinedTypeLinkNode::new("loopA"),
        ))
        .add_defined_type(DefinedTypeNode::new(
            "intoLoop",
            DefinedTypeLinkNode::new("loopA"),
        ))
}

fn other_program() -> ProgramNode {
    ProgramNode::new("other", "Other11111111111111111111111111111111111111")
        .add_defined_type(DefinedTypeNode::new("rate", IntegerTypeNode::le(U64)))
        .add_defined_type(DefinedTypeNode::new("price", FloatTypeNode::le(F64)))
        .add_defined_type(DefinedTypeNode::new(
            "spotPrice",
            DefinedTypeLinkNode::new("price"),
        ))
        .add_defined_type(DefinedTypeNode::new(
            "mainRate",
            type_link_with_program("rate", "main"),
        ))
}

#[test]
fn linked_defaults_become_floats() {
    for r#type in [
        TypeNode::from(DefinedTypeLinkNode::new("rate")),
        DefinedTypeLinkNode::new("feeRate").into(),
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
        let account = account_with_field(DefinedTypeLinkNode::new(identifier), integer());
        let root = root_with_programs(main_program().add_account(account), other_program());
        assert_eq!(transform_integers_to_floats(root.clone()), root);
    }
}

#[test]
fn dangling_links_keep_the_integer() {
    for r#type in [
        DefinedTypeLinkNode::new("missing"),
        DefinedTypeLinkNode::new("price"),
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
        (DefinedTypeLinkNode::new("count"), integer()),
        (type_link_with_program("rate", "other"), integer()),
        (
            DefinedTypeLinkNode::new("count"),
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
            ..StructFieldTypeNode::new(identifier, DefinedTypeLinkNode::new(identifier))
        };
        AccountNode::new(
            "quote",
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
