use codama_errors::CodamaResult;
use codama_korok_visitors::{IdentifyFieldTypesVisitor, KorokVisitable, SetInstructionsVisitor};
use codama_koroks::{EnumKorok, StructKorok};
use codama_nodes::{
    InstructionArgumentNode, InstructionNode, NumberFormat::U64, NumberTypeNode, ProgramNode,
};

#[test]
fn from_struct_with_program_directive() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaInstruction)]
        #[codama(program(name = "externalProgram", address = "ExtProg111111111111111111111111111111111111"))]
        struct Transfer {
            amount: u64,
        }
    };
    let mut korok = StructKorok::parse(&item)?;

    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut SetInstructionsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            ProgramNode {
                name: "externalProgram".into(),
                public_key: "ExtProg111111111111111111111111111111111111".to_string(),
                instructions: vec![InstructionNode {
                    name: "transfer".into(),
                    arguments: vec![InstructionArgumentNode::new(
                        "amount",
                        NumberTypeNode::le(U64)
                    )],
                    ..InstructionNode::default()
                }],
                ..ProgramNode::default()
            }
            .into()
        )
    );
    Ok(())
}

#[test]
fn from_enum_with_program_directive() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaInstructions)]
        #[codama(program(name = "externalProgram", address = "ExtProg111111111111111111111111111111111111"))]
        enum ExternalInstructions {
            Transfer { amount: u64 },
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut SetInstructionsVisitor::new())?;

    let program = match &korok.node {
        Some(codama_nodes::Node::Program(p)) => p,
        _ => panic!("expected ProgramNode"),
    };
    assert_eq!(program.name, "externalProgram".into());
    assert_eq!(
        program.public_key,
        "ExtProg111111111111111111111111111111111111"
    );
    assert_eq!(program.instructions.len(), 1);
    assert_eq!(program.instructions[0].name, "transfer".into());
    Ok(())
}
