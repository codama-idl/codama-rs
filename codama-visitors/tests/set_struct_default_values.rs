use codama_nodes::{
    DefaultValueStrategy, DefinedTypeNode, InstructionArgumentNode, InstructionInputValueNode,
    InstructionNode, NumberTypeNode, NumberValueNode, ProgramNode, RootNode, StructFieldTypeNode,
    StructTypeNode, TypeNode, ValueNode, U8,
};
use codama_visitors::{set_struct_default_values, StructDefaultValue, TransformVisitor};
use pretty_assertions::assert_eq;

fn sample_root() -> RootNode {
    let data = StructTypeNode::new(vec![
        StructFieldTypeNode::new("bump", NumberTypeNode::le(U8)),
        StructFieldTypeNode::new("count", NumberTypeNode::le(U8)),
    ]);
    let mut program = ProgramNode::new("p", "Myprogram1111111111111111111111111111111111");
    program
        .defined_types
        .push(DefinedTypeNode::new("config", data));
    RootNode::new(program)
}

fn field_default(root: &RootNode, index: usize) -> Option<ValueNode> {
    match &*root.program.defined_types[0].r#type {
        TypeNode::Struct(s) => (*s.fields[index].default_value).clone(),
        other => panic!("expected struct, got {other:?}"),
    }
}

#[test]
fn sets_default_values_for_named_fields() {
    let root = set_struct_default_values([(
        "[definedTypeNode]config",
        vec![(
            "bump".to_string(),
            StructDefaultValue::Value(NumberValueNode::new(255u8).into()),
        )],
    )])
    .visit_root(sample_root());

    assert_eq!(
        field_default(&root, 0),
        Some(ValueNode::Number(NumberValueNode::new(255u8))),
    );
    // `count` was not in the map, so it keeps no default.
    assert_eq!(field_default(&root, 1), None);
}

#[test]
fn supports_strategy_and_clear() {
    let mut data = StructTypeNode::new(vec![StructFieldTypeNode::new(
        "bump",
        NumberTypeNode::le(U8),
    )]);
    // `count` starts with a default we will clear.
    let mut count = StructFieldTypeNode::new("count", NumberTypeNode::le(U8));
    count.default_value = Box::new(Some(NumberValueNode::new(1u8).into()));
    data.fields.push(count);
    let mut program = ProgramNode::new("p", "Myprogram1111111111111111111111111111111111");
    program
        .defined_types
        .push(DefinedTypeNode::new("config", data));

    let root = set_struct_default_values([(
        "[definedTypeNode]config",
        vec![
            (
                "bump".to_string(),
                StructDefaultValue::WithStrategy(
                    NumberValueNode::new(0u8).into(),
                    Some(DefaultValueStrategy::Omitted),
                ),
            ),
            ("count".to_string(), StructDefaultValue::Clear),
        ],
    )])
    .visit_root(RootNode::new(program));

    let TypeNode::Struct(s) = &*root.program.defined_types[0].r#type else {
        panic!("expected struct")
    };
    assert_eq!(
        *s.fields[0].default_value,
        Some(ValueNode::Number(NumberValueNode::new(0u8)))
    );
    assert_eq!(
        s.fields[0].default_value_strategy,
        Some(DefaultValueStrategy::Omitted)
    );
    // `count`'s pre-existing default was cleared.
    assert_eq!(*s.fields[1].default_value, None);
}

#[test]
fn sets_default_values_on_instruction_arguments() {
    let mut ix = InstructionNode {
        name: "transfer".into(),
        ..Default::default()
    };
    ix.arguments.push(InstructionArgumentNode::new(
        "amount",
        NumberTypeNode::le(U8),
    ));
    ix.extra_arguments
        .push(InstructionArgumentNode::new("bump", NumberTypeNode::le(U8)));
    let mut program = ProgramNode::new("p", "Myprogram1111111111111111111111111111111111");
    program.instructions.push(ix);

    let root = set_struct_default_values([(
        "transfer",
        vec![
            (
                "amount".to_string(),
                StructDefaultValue::Value(NumberValueNode::new(42u8).into()),
            ),
            (
                "bump".to_string(),
                StructDefaultValue::Value(NumberValueNode::new(254u8).into()),
            ),
        ],
    )])
    .visit_root(RootNode::new(program));

    let ix = &root.program.instructions[0];
    assert_eq!(
        *ix.arguments[0].default_value,
        Some(InstructionInputValueNode::NumberValue(
            NumberValueNode::new(42u8)
        ))
    );
    // extraArguments are covered too.
    assert_eq!(
        *ix.extra_arguments[0].default_value,
        Some(InstructionInputValueNode::NumberValue(
            NumberValueNode::new(254u8)
        ))
    );
}
