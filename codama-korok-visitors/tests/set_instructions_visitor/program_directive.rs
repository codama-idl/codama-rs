use codama_errors::CodamaResult;
use codama_korok_visitors::{IdentifyFieldTypesVisitor, KorokVisitable, SetInstructionsVisitor};
use codama_koroks::{EnumKorok, StructKorok};
use codama_nodes::test_utils::id;
use codama_nodes::{
    InstructionNode, IntegerFormat::U64, IntegerTypeNode, ProgramNode, StructFieldTypeNode,
    StructTypeNode,
};

#[test]
fn from_struct_with_program_directive() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaInstruction)]
        #[codama(program(identifier = "externalProgram", address = "ExtProg111111111111111111111111111111111111"))]
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
                identifier: id("externalProgram"),
                public_key: "ExtProg111111111111111111111111111111111111".to_string(),
                instructions: vec![InstructionNode {
                    identifier: id("transfer"),
                    data: Box::new(Some(
                        StructTypeNode::new(vec![StructFieldTypeNode::new(
                            id("amount"),
                            IntegerTypeNode::le(U64)
                        )])
                        .into()
                    )),
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
        #[codama(program(identifier = "externalProgram", address = "ExtProg111111111111111111111111111111111111"))]
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
    assert_eq!(program.identifier, id("externalProgram"));
    assert_eq!(
        program.public_key,
        "ExtProg111111111111111111111111111111111111"
    );
    assert_eq!(program.instructions.len(), 1);
    assert_eq!(program.instructions[0].identifier, id("transfer"));
    Ok(())
}
