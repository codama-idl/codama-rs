use super::utils::{
    account_with_type, constant_with_defined_type_link, float, integer,
    program_with_float_defined_type, root_with_account, transform_integers_to_floats,
};
use codama_nodes::{
    AccountNode, ArrayTypeNode, BooleanTypeNode, BytesEncoding, ConstantDiscriminatorNode,
    ConstantNode, ConstantValueNode, DateTimeTypeNode, DefinedTypeLinkNode, DiscriminatorNode,
    DurationTypeNode, EnumTypeNode, EventNode, FieldDiscriminatorNode, FixedPointTypeNode,
    FloatFormat::{F32, F64},
    FloatTypeNode, FloatValueNode, HasTransforms, HiddenPrefixTransformNode,
    HiddenSuffixTransformNode, InstructionNode,
    IntegerFormat::{U64, U8},
    IntegerTypeNode, IntegerValueNode, MapTypeNode, OptionTypeNode, Path, ProgramNode, RootNode,
    SentinelCountNode, SentinelTransformNode, SetTypeNode, SizeDiscriminatorNode,
    SizePrefixTransformNode, StringTypeNode, StringValueNode, StructTypeNode, TypeNode, ValueNode,
    ZeroableOptionTypeNode,
};

fn integer_with_constant(value: ValueNode) -> IntegerTypeNode {
    IntegerTypeNode::le(U8).add_transform(HiddenPrefixTransformNode::new(vec![
        constant_with_defined_type_link("rate", value),
    ]))
}

fn string() -> StringTypeNode {
    StringTypeNode::utf8()
}

fn constant_discriminator(r#type: TypeNode, value: ValueNode) -> DiscriminatorNode {
    ConstantDiscriminatorNode::new(ConstantValueNode::new(r#type, value), 0).into()
}

fn account_with_discriminator(discriminator: DiscriminatorNode) -> AccountNode {
    AccountNode {
        discriminators: vec![discriminator],
        ..AccountNode::new("config", StructTypeNode::new(vec![]))
    }
}

fn instruction_with_discriminator(discriminator: DiscriminatorNode) -> InstructionNode {
    InstructionNode {
        identifier: "update".into(),
        discriminators: vec![discriminator],
        ..InstructionNode::default()
    }
}

#[test]
fn constants_in_transforms_become_floats() {
    let types_with_constant: [fn(ValueNode) -> TypeNode; 6] = [
        |value| {
            string()
                .add_transform(HiddenPrefixTransformNode::new(vec![
                    ConstantValueNode::new(FloatTypeNode::le(F32), value),
                ]))
                .into()
        },
        |value| {
            string()
                .add_transform(HiddenPrefixTransformNode::new(vec![
                    ConstantValueNode::new(FloatTypeNode::le(F64), value),
                ]))
                .into()
        },
        |value| {
            string()
                .add_transform(HiddenSuffixTransformNode::new(vec![
                    constant_with_defined_type_link("rate", value),
                ]))
                .into()
        },
        |value| {
            string()
                .add_transform(SentinelTransformNode::new(constant_with_defined_type_link(
                    "rate", value,
                )))
                .into()
        },
        |value| {
            DefinedTypeLinkNode::new("rate")
                .add_transform(SentinelTransformNode::new(constant_with_defined_type_link(
                    "rate", value,
                )))
                .into()
        },
        |value| {
            let end = ConstantValueNode::new(
                string().add_transform(HiddenPrefixTransformNode::new(vec![
                    constant_with_defined_type_link("rate", value),
                ])),
                StringValueNode::new("end"),
            );
            string()
                .add_transform(SentinelTransformNode::new(end))
                .into()
        },
    ];
    for type_with_constant in types_with_constant {
        for (written, expected) in [
            (integer(), float()),
            (
                IntegerValueNode::new(-2i64).into(),
                FloatValueNode::new(-2.0).into(),
            ),
        ] {
            let root = |value| {
                root_with_account(
                    program_with_float_defined_type("rate"),
                    account_with_type(type_with_constant(value)),
                )
            };
            assert_eq!(transform_integers_to_floats(root(written)), root(expected));
        }
    }
}

#[test]
fn constants_in_nested_integer_types_become_floats() {
    let types_with_nested_constant: [fn(ValueNode) -> TypeNode; 13] = [
        |value| integer_with_constant(value).into(),
        |value| {
            OptionTypeNode {
                prefix: integer_with_constant(value),
                ..OptionTypeNode::new(string())
            }
            .into()
        },
        |value| BooleanTypeNode::new(integer_with_constant(value)).into(),
        |value| {
            EnumTypeNode {
                size: integer_with_constant(value),
                ..EnumTypeNode::new(vec![])
            }
            .into()
        },
        |value| FixedPointTypeNode::new(integer_with_constant(value), 2).into(),
        |value| DateTimeTypeNode::new(integer_with_constant(value)).into(),
        |value| DurationTypeNode::new(integer_with_constant(value)).into(),
        |value| ArrayTypeNode::prefixed(string(), integer_with_constant(value)).into(),
        |value| SetTypeNode::prefixed(string(), integer_with_constant(value)).into(),
        |value| MapTypeNode::prefixed(string(), string(), integer_with_constant(value)).into(),
        |value| {
            string()
                .add_transform(SizePrefixTransformNode::new(integer_with_constant(value)))
                .into()
        },
        |value| {
            ArrayTypeNode::new(
                string(),
                SentinelCountNode::new(constant_with_defined_type_link("rate", value)),
            )
            .into()
        },
        |value| {
            ZeroableOptionTypeNode::custom(string(), constant_with_defined_type_link("rate", value))
                .into()
        },
    ];
    for type_with_nested_constant in types_with_nested_constant {
        let root = |value| {
            root_with_account(
                program_with_float_defined_type("rate"),
                account_with_type(type_with_nested_constant(value)),
            )
        };
        assert_eq!(transform_integers_to_floats(root(integer())), root(float()));
    }
}

#[test]
fn program_constants_become_floats() {
    for r#type in [
        TypeNode::from(FloatTypeNode::le(F64)),
        DefinedTypeLinkNode::new("rate").into(),
    ] {
        let root = |value| {
            RootNode::new(
                program_with_float_defined_type("rate").add_constant(ConstantNode::new(
                    "fee",
                    r#type.clone(),
                    value,
                )),
            )
        };
        assert_eq!(transform_integers_to_floats(root(integer())), root(float()));
    }
}

#[test]
fn constant_discriminators_become_floats() {
    let programs_with_discriminator: [fn(DiscriminatorNode) -> ProgramNode; 4] = [
        |discriminator| {
            program_with_float_defined_type("rate")
                .add_account(account_with_discriminator(discriminator))
        },
        |discriminator| {
            program_with_float_defined_type("rate").add_event(EventNode {
                discriminators: vec![discriminator],
                ..EventNode::new("updated", StructTypeNode::new(vec![]))
            })
        },
        |discriminator| {
            program_with_float_defined_type("rate")
                .add_instruction(instruction_with_discriminator(discriminator))
        },
        |discriminator| {
            program_with_float_defined_type("rate").add_instruction(InstructionNode {
                identifier: "batch".into(),
                sub_instructions: vec![instruction_with_discriminator(discriminator)],
                ..InstructionNode::default()
            })
        },
    ];
    for program_with_discriminator in programs_with_discriminator {
        for r#type in [
            TypeNode::from(FloatTypeNode::le(F64)),
            DefinedTypeLinkNode::new("rate").into(),
        ] {
            let root = |value| {
                RootNode::new(program_with_discriminator(constant_discriminator(
                    r#type.clone(),
                    value,
                )))
            };
            assert_eq!(transform_integers_to_floats(root(integer())), root(float()));
        }
    }
}

#[test]
fn discriminators_at_other_types_keep_their_value() {
    for discriminator in [
        ConstantDiscriminatorNode::new(
            ConstantValueNode::bytes(BytesEncoding::Base16, "01020304"),
            0,
        )
        .into(),
        constant_discriminator(IntegerTypeNode::le(U8).into(), integer()),
        constant_discriminator(DefinedTypeLinkNode::new("missing").into(), integer()),
        FieldDiscriminatorNode::new(Path::try_from("kind").unwrap(), 0).into(),
        SizeDiscriminatorNode::new(8).into(),
    ] {
        let root = RootNode::new(
            program_with_float_defined_type("rate")
                .add_account(account_with_discriminator(discriminator)),
        );
        assert_eq!(transform_integers_to_floats(root.clone()), root);
    }
}

#[test]
fn constants_at_other_types_keep_their_value() {
    for (r#type, value) in [
        (TypeNode::from(IntegerTypeNode::le(U64)), integer()),
        (
            IntegerTypeNode::le(U64).into(),
            FloatValueNode::new(2.0).into(),
        ),
        (DefinedTypeLinkNode::new("missing").into(), integer()),
        (string().into(), StringValueNode::new("1").into()),
    ] {
        let constant = ConstantValueNode::new(r#type.clone(), value.clone());
        let string_with_sentinel = string().add_transform(SentinelTransformNode::new(constant));
        let root = RootNode::new(
            program_with_float_defined_type("rate")
                .add_constant(ConstantNode::new("fee", r#type, value))
                .add_account(account_with_type(string_with_sentinel)),
        );
        assert_eq!(transform_integers_to_floats(root.clone()), root);
    }
}
