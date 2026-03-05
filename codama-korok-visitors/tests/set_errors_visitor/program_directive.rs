use codama_errors::CodamaResult;
use codama_korok_visitors::{KorokVisitable, SetErrorsVisitor};
use codama_koroks::EnumKorok;
use codama_nodes::{ErrorNode, ProgramNode};

#[test]
fn from_enum_with_program_directive() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaErrors)]
        #[codama(program(name = "externalProgram", address = "ExtProg111111111111111111111111111111111111"))]
        enum ExternalErrors {
            #[error("Something went wrong")]
            SomeError,
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

    korok.accept(&mut SetErrorsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            ProgramNode {
                name: "externalProgram".into(),
                public_key: "ExtProg111111111111111111111111111111111111".to_string(),
                errors: vec![ErrorNode::new("someError", 0, "Something went wrong")],
                ..ProgramNode::default()
            }
            .into()
        )
    );
    Ok(())
}
