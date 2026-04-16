use codama_errors::CodamaResult;
use codama_korok_visitors::{IdentifyFieldTypesVisitor, KorokVisitable, SetEventsVisitor};
use codama_koroks::{EnumKorok, StructKorok};
use codama_nodes::{
    EventNode, NumberFormat::U64, NumberTypeNode, ProgramNode, PublicKeyTypeNode,
    StructFieldTypeNode, StructTypeNode,
};

#[test]
fn from_struct_with_program_directive() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaEvent)]
        #[codama(program(name = "externalProgram", address = "ExtProg111111111111111111111111111111111111"))]
        struct TransferEvent {
            authority: Pubkey,
            amount: u64,
        }
    };
    let mut korok = StructKorok::parse(&item)?;

    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut SetEventsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            ProgramNode {
                name: "externalProgram".into(),
                public_key: "ExtProg111111111111111111111111111111111111".to_string(),
                events: vec![EventNode::new(
                    "transferEvent",
                    StructTypeNode::new(vec![
                        StructFieldTypeNode::new("authority", PublicKeyTypeNode::new()),
                        StructFieldTypeNode::new("amount", NumberTypeNode::le(U64)),
                    ])
                )],
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
        #[derive(CodamaEvents)]
        #[codama(program(name = "externalProgram", address = "ExtProg111111111111111111111111111111111111"))]
        enum ExternalEvents {
            Transfer,
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

    korok.accept(&mut SetEventsVisitor::new())?;

    let program = match &korok.node {
        Some(codama_nodes::Node::Program(p)) => p,
        _ => panic!("expected ProgramNode"),
    };
    assert_eq!(program.name, "externalProgram".into());
    assert_eq!(
        program.public_key,
        "ExtProg111111111111111111111111111111111111"
    );
    assert_eq!(program.events.len(), 1);
    assert_eq!(program.events[0].name, "transfer".into());
    Ok(())
}
