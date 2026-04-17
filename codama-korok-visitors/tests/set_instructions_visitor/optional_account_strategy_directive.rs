use codama_errors::CodamaResult;
use codama_korok_visitors::{IdentifyFieldTypesVisitor, KorokVisitable, SetInstructionsVisitor};
use codama_koroks::{EnumKorok, StructKorok};
use codama_nodes::{
    InstructionNode, InstructionOptionalAccountStrategy, NumberFormat::U64, NumberTypeNode,
};

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
                optional_account_strategy: InstructionOptionalAccountStrategy::Omitted,
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
        InstructionOptionalAccountStrategy::Omitted
    );
    Ok(())
}
