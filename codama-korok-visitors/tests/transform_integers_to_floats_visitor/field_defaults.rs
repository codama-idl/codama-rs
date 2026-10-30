use super::utils::{
    account_with_field, field, float, integer, root_with_account, transform_integers_to_floats,
};
use codama_nodes::{
    AccountNode, ArrayTypeNode, ArrayValueNode, DateTimeTypeNode, Decimal, DefinedTypeNode,
    DurationTypeNode, EnumTypeNode, EnumVariantTypeNode, EventNode, FixedPointTypeNode,
    FixedSizeTransformNode,
    FloatFormat::{F32, F64},
    FloatTypeNode, FloatValueNode, HasTransforms, InstructionNode,
    IntegerFormat::{I64, U64, U8},
    IntegerTypeNode, IntegerValueNode, MapTypeNode, OptionTypeNode, ProgramNode,
    RemainderOptionTypeNode, RootNode, SetTypeNode, SomeValueNode, StringValueNode,
    StructFieldTypeNode, StructTypeNode, TupleTypeNode, TypeNode, ValueNode,
    ZeroableOptionTypeNode,
};

fn data(value: ValueNode) -> StructTypeNode {
    StructTypeNode::new(vec![field(FloatTypeNode::le(F64), value)])
}

fn instruction(data: StructTypeNode) -> InstructionNode {
    InstructionNode {
        identifier: "update".into(),
        data: Box::new(Some(data.into())),
        ..InstructionNode::default()
    }
}

#[test]
fn integer_defaults_at_float_types_become_floats() {
    for r#type in [
        TypeNode::from(FloatTypeNode::le(F32)),
        FloatTypeNode::le(F64).into(),
        FloatTypeNode::le(F64)
            .add_transform(FixedSizeTransformNode::new(8))
            .into(),
    ] {
        let root = |value| {
            root_with_account(
                ProgramNode::default(),
                account_with_field(r#type.clone(), value),
            )
        };
        assert_eq!(transform_integers_to_floats(root(integer())), root(float()));
    }
}

#[test]
fn floats_keep_every_digit_of_the_integer() {
    for (integer, digits) in [
        (IntegerValueNode::new(-5i64), "-5"),
        (IntegerValueNode::new(u64::MAX), "18446744073709551615"),
        (
            IntegerValueNode::new(9_007_199_254_740_993u64),
            "9007199254740993",
        ),
    ] {
        let root = |value| {
            root_with_account(
                ProgramNode::default(),
                account_with_field(FloatTypeNode::le(F64), value),
            )
        };
        let float = FloatValueNode::new(Decimal::try_from(digits).unwrap());
        assert_eq!(
            transform_integers_to_floats(root(integer.into())),
            root(float.into())
        );
    }
}

#[test]
fn other_defaults_keep_their_kind() {
    for (value, r#type) in [
        (integer(), TypeNode::from(IntegerTypeNode::le(U64))),
        (
            integer(),
            FixedPointTypeNode::new(IntegerTypeNode::le(U64), 6).into(),
        ),
        (
            integer(),
            DateTimeTypeNode::new(IntegerTypeNode::le(I64)).into(),
        ),
        (
            integer(),
            DurationTypeNode::new(IntegerTypeNode::le(U64)).into(),
        ),
        (
            integer(),
            OptionTypeNode::new(FloatTypeNode::le(F64)).into(),
        ),
        (
            FloatValueNode::new(1.5).into(),
            FloatTypeNode::le(F64).into(),
        ),
        (
            FloatValueNode::new(2.0).into(),
            IntegerTypeNode::le(U64).into(),
        ),
        (
            StringValueNode::new("1").into(),
            FloatTypeNode::le(F64).into(),
        ),
        (
            SomeValueNode::new(IntegerValueNode::new(1u64)).into(),
            OptionTypeNode::new(FloatTypeNode::le(F64)).into(),
        ),
        (
            ArrayValueNode::new(vec![integer()]).into(),
            ArrayTypeNode::fixed(FloatTypeNode::le(F64), 1).into(),
        ),
    ] {
        let root = root_with_account(ProgramNode::default(), account_with_field(r#type, value));
        assert_eq!(transform_integers_to_floats(root.clone()), root);
    }
}

#[test]
fn defaults_in_every_type_tree_become_floats() {
    let programs_with_data: [fn(ValueNode) -> ProgramNode; 6] = [
        |value| ProgramNode::default().add_account(AccountNode::new("config", data(value))),
        |value| {
            ProgramNode::default().add_defined_type(DefinedTypeNode::new("settings", data(value)))
        },
        |value| ProgramNode::default().add_event(EventNode::new("updated", data(value))),
        |value| ProgramNode::default().add_instruction(instruction(data(value))),
        |value| {
            ProgramNode::default().add_instruction(InstructionNode {
                return_data: Box::new(Some(data(value).into())),
                ..instruction(StructTypeNode::new(vec![]))
            })
        },
        |value| {
            ProgramNode::default().add_instruction(InstructionNode {
                sub_instructions: vec![instruction(data(value))],
                ..instruction(StructTypeNode::new(vec![]))
            })
        },
    ];
    for program_with_data in programs_with_data {
        assert_eq!(
            transform_integers_to_floats(RootNode::new(program_with_data(integer()))),
            RootNode::new(program_with_data(float()))
        );
    }
}

#[test]
fn defaults_at_any_depth_become_floats() {
    let types_with_nested_data: [fn(ValueNode) -> TypeNode; 10] = [
        |value| StructTypeNode::new(vec![StructFieldTypeNode::new("inner", data(value))]).into(),
        |value| TupleTypeNode::new(vec![data(value).into()]).into(),
        |value| ArrayTypeNode::fixed(data(value), 2).into(),
        |value| SetTypeNode::fixed(data(value), 2).into(),
        |value| MapTypeNode::fixed(data(value), IntegerTypeNode::le(U8), 2).into(),
        |value| MapTypeNode::fixed(IntegerTypeNode::le(U8), data(value), 2).into(),
        |value| OptionTypeNode::new(data(value)).into(),
        |value| RemainderOptionTypeNode::new(data(value)).into(),
        |value| ZeroableOptionTypeNode::new(data(value)).into(),
        |value| EnumTypeNode::new(vec![EnumVariantTypeNode::with_data("a", data(value))]).into(),
    ];
    for type_with_nested_data in types_with_nested_data {
        let root = |value| {
            RootNode::new(
                ProgramNode::default()
                    .add_defined_type(DefinedTypeNode::new("holder", type_with_nested_data(value))),
            )
        };
        assert_eq!(transform_integers_to_floats(root(integer())), root(float()));
    }
}
