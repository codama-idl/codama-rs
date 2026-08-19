use codama_errors::CodamaResult;
use codama_korok_visitors::{IdentifyFieldTypesVisitor, KorokVisitable, SetInstructionsVisitor};
use codama_koroks::{EnumKorok, StructKorok};
use codama_nodes::{InstructionNode, NumberFormat::U64, NumberTypeNode, OptionalAccountStrategy};

#[test]
fn from_struct_with_optional_account_strategy_directive() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaInstruction)]
        #[codama(optional_account_strategy = omitted)]
        struct Initialize {
            amount: u64,
        }
    };
    let mut korok = StructKorok::parse(&item)?;

    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut SetInstructionsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            InstructionNode {
                name: "initialize".into(),
                optional_account_strategy: Some(OptionalAccountStrategy::Omitted),
                arguments: vec![codama_nodes::InstructionArgumentNode::new(
                    "amount",
                    NumberTypeNode::le(U64)
                )],
                ..InstructionNode::default()
            }
            .into()
        )
    );
    Ok(())
}

#[test]
fn from_enum_variant_with_optional_account_strategy_directive() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaInstructions)]
        enum MyProgramInstructions {
            #[codama(optional_account_strategy = omitted)]
            Initialize { amount: u64 },
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut SetInstructionsVisitor::new())?;

    let program = match &korok.node {
        Some(codama_nodes::Node::Program(program)) => program,
        _ => panic!("expected ProgramNode"),
    };
    assert_eq!(
        program.instructions[0].optional_account_strategy,
        Some(OptionalAccountStrategy::Omitted)
    );
    Ok(())
}

#[test]
fn from_enum_with_optional_account_strategy_directive() -> CodamaResult<()> {
    // The enum-level directive applies to all generated instructions.
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaInstructions)]
        #[codama(optional_account_strategy = omitted)]
        enum MyProgramInstructions {
            Initialize { amount: u64 },
            Close,
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut SetInstructionsVisitor::new())?;

    let program = match &korok.node {
        Some(codama_nodes::Node::Program(program)) => program,
        _ => panic!("expected ProgramNode"),
    };
    assert_eq!(
        program.instructions[0].optional_account_strategy,
        Some(OptionalAccountStrategy::Omitted)
    );
    assert_eq!(
        program.instructions[1].optional_account_strategy,
        Some(OptionalAccountStrategy::Omitted)
    );
    Ok(())
}

#[test]
fn variant_directive_overrides_the_enum_directive() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaInstructions)]
        #[codama(optional_account_strategy = omitted)]
        enum MyProgramInstructions {
            #[codama(optional_account_strategy = program_id)]
            Initialize { amount: u64 },
            Close,
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut SetInstructionsVisitor::new())?;

    let program = match &korok.node {
        Some(codama_nodes::Node::Program(program)) => program,
        _ => panic!("expected ProgramNode"),
    };
    assert_eq!(
        program.instructions[0].optional_account_strategy,
        Some(OptionalAccountStrategy::ProgramId)
    );
    assert_eq!(
        program.instructions[1].optional_account_strategy,
        Some(OptionalAccountStrategy::Omitted)
    );
    Ok(())
}

#[test]
fn enums_without_the_directive_are_unaffected_by_previous_enums() -> CodamaResult<()> {
    // The stashed enum-level strategy must not leak into subsequent enums
    // visited by the same visitor instance.
    let first: syn::Item = syn::parse_quote! {
        #[derive(CodamaInstructions)]
        #[codama(optional_account_strategy = omitted)]
        enum FirstInstructions {
            Initialize,
        }
    };
    let second: syn::Item = syn::parse_quote! {
        #[derive(CodamaInstructions)]
        enum SecondInstructions {
            Close,
        }
    };
    let mut first_korok = EnumKorok::parse(&first)?;
    let mut second_korok = EnumKorok::parse(&second)?;

    let mut visitor = SetInstructionsVisitor::new();
    first_korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    first_korok.accept(&mut visitor)?;
    second_korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    second_korok.accept(&mut visitor)?;

    let program = match &second_korok.node {
        Some(codama_nodes::Node::Program(program)) => program,
        _ => panic!("expected ProgramNode"),
    };
    assert_eq!(program.instructions[0].optional_account_strategy, None);
    Ok(())
}
