use codama_nodes::{
    DefaultValueStrategy, DefinedTypeLinkNode, DefinedTypeNode, EnumEmptyVariantTypeNode,
    EnumStructVariantTypeNode, EnumTupleVariantTypeNode, EnumTypeNode, InstructionArgumentNode,
    InstructionInputValueNode, InstructionNode, NumberTypeNode, ProgramNode, RootNode,
    StructFieldTypeNode, StructTypeNode, TupleTypeNode, TypeNode, U64, U8,
};
use codama_visitors::create_sub_instructions_from_enum_args;
use pretty_assertions::assert_eq;

fn arg(name: &str, type_node: impl Into<TypeNode>) -> InstructionArgumentNode {
    InstructionArgumentNode::new(name, type_node)
}

fn arg_names(ix: &InstructionNode) -> Vec<String> {
    ix.arguments.iter().map(|a| a.name.to_string()).collect()
}

#[test]
fn splits_empty_variants_into_sub_instructions_with_a_discriminator() {
    let enum_type = EnumTypeNode::new(vec![
        EnumEmptyVariantTypeNode::new("add").into(),
        EnumEmptyVariantTypeNode::new("remove").into(),
    ]);
    let mut ix = InstructionNode {
        name: "update".into(),
        ..Default::default()
    };
    ix.arguments.push(arg("action", enum_type));
    let mut program = ProgramNode::new("p", "Myprogram1111111111111111111111111111111111");
    program.instructions.push(ix);

    let root =
        create_sub_instructions_from_enum_args(RootNode::new(program), [("update", "action")]);
    let parent = &root.program.instructions[0];

    // Parent keeps its enum argument and gains two sub-instructions.
    assert_eq!(arg_names(parent), vec!["action"]);
    assert_eq!(parent.sub_instructions.len(), 2);

    let add = &parent.sub_instructions[0];
    assert_eq!(add.name.as_ref(), "updateAdd");
    assert_eq!(arg_names(add), vec!["updateAddDiscriminator"]);

    // The discriminator is an omitted u8 defaulting to the variant index.
    let discriminator = &add.arguments[0];
    assert_eq!(
        discriminator.default_value_strategy,
        Some(DefaultValueStrategy::Omitted)
    );
    assert_eq!(
        *discriminator.r#type,
        TypeNode::Number(NumberTypeNode::le(U8))
    );
    match discriminator.default_value.as_ref() {
        Some(InstructionInputValueNode::NumberValue(n)) => {
            assert_eq!(n.number, 0u32.into())
        }
        other => panic!("expected numberValueNode, got {other:?}"),
    }

    let remove = &parent.sub_instructions[1];
    assert_eq!(remove.name.as_ref(), "updateRemove");
    match remove.arguments[0].default_value.as_ref() {
        Some(InstructionInputValueNode::NumberValue(n)) => {
            assert_eq!(n.number, 1u32.into())
        }
        other => panic!("expected numberValueNode, got {other:?}"),
    }
}

#[test]
fn flattens_struct_variants_and_keeps_tuple_variants() {
    let enum_type = EnumTypeNode::new(vec![
        EnumStructVariantTypeNode::new(
            "deposit",
            StructTypeNode::new(vec![StructFieldTypeNode::new(
                "amount",
                NumberTypeNode::le(U64),
            )]),
        )
        .into(),
        EnumTupleVariantTypeNode::new(
            "withdraw",
            TupleTypeNode::new(vec![NumberTypeNode::le(U64).into()]),
        )
        .into(),
    ]);
    let mut ix = InstructionNode {
        name: "bank".into(),
        ..Default::default()
    };
    ix.arguments.push(arg("action", enum_type));
    let mut program = ProgramNode::new("p", "Myprogram1111111111111111111111111111111111");
    program.instructions.push(ix);

    let root = create_sub_instructions_from_enum_args(RootNode::new(program), [("bank", "action")]);
    let subs = &root.program.instructions[0].sub_instructions;

    // Struct variant: the struct is flattened into its fields.
    let deposit = &subs[0];
    assert_eq!(
        arg_names(deposit),
        vec!["bankDepositDiscriminator", "amount"]
    );

    // Tuple variant: kept as a single tuple-typed argument named like the enum arg.
    let withdraw = &subs[1];
    assert_eq!(
        arg_names(withdraw),
        vec!["bankWithdrawDiscriminator", "action"]
    );
    assert!(matches!(*withdraw.arguments[1].r#type, TypeNode::Tuple(_)));
}

#[test]
fn preserves_arguments_before_and_after_the_enum_arg() {
    let enum_type = EnumTypeNode::new(vec![EnumEmptyVariantTypeNode::new("go").into()]);
    let mut ix = InstructionNode {
        name: "run".into(),
        ..Default::default()
    };
    ix.arguments.push(arg("before", NumberTypeNode::le(U8)));
    ix.arguments.push(arg("action", enum_type));
    ix.arguments.push(arg("after", NumberTypeNode::le(U8)));
    let mut program = ProgramNode::new("p", "Myprogram1111111111111111111111111111111111");
    program.instructions.push(ix);

    let root = create_sub_instructions_from_enum_args(RootNode::new(program), [("run", "action")]);
    let go = &root.program.instructions[0].sub_instructions[0];

    assert_eq!(arg_names(go), vec!["before", "runGoDiscriminator", "after"]);
}

#[test]
fn resolves_the_enum_through_a_defined_type_link() {
    let enum_type = EnumTypeNode::new(vec![
        EnumEmptyVariantTypeNode::new("add").into(),
        EnumEmptyVariantTypeNode::new("remove").into(),
    ]);
    let mut ix = InstructionNode {
        name: "update".into(),
        ..Default::default()
    };
    ix.arguments.push(arg(
        "action",
        DefinedTypeLinkNode {
            name: "myEnum".into(),
            program: None,
        },
    ));
    let mut program = ProgramNode::new("p", "Myprogram1111111111111111111111111111111111");
    program
        .defined_types
        .push(DefinedTypeNode::new("myEnum", enum_type));
    program.instructions.push(ix);

    let root =
        create_sub_instructions_from_enum_args(RootNode::new(program), [("update", "action")]);
    let subs = &root.program.instructions[0].sub_instructions;

    assert_eq!(subs.len(), 2);
    assert_eq!(subs[0].name.as_ref(), "updateAdd");
}

#[test]
fn leaves_the_instruction_unchanged_when_the_argument_is_missing() {
    let mut ix = InstructionNode {
        name: "update".into(),
        ..Default::default()
    };
    ix.arguments.push(arg("amount", NumberTypeNode::le(U8)));
    let mut program = ProgramNode::new("p", "Myprogram1111111111111111111111111111111111");
    program.instructions.push(ix);

    let root =
        create_sub_instructions_from_enum_args(RootNode::new(program), [("update", "action")]);
    let parent = &root.program.instructions[0];

    assert!(parent.sub_instructions.is_empty());
    assert_eq!(arg_names(parent), vec!["amount"]);
}
