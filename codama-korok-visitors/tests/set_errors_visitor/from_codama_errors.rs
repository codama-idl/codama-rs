use codama_errors::CodamaResult;
use codama_korok_visitors::{KorokVisitable, SetBorshTypesVisitor, SetErrorsVisitor};
use codama_koroks::{EnumKorok, StructKorok};
use codama_nodes::{BooleanTypeNode, ErrorNode, ProgramNode};

#[test]
fn from_enum() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaErrors)]
        enum MyProgramErrors {
            #[error("Lamports below rent-exempt threshold")]
            NotRentExempt,
            #[error("Insufficient funds")]
            InsufficientFunds {},
            #[error("Invalid Mint")]
            InvalidMint { mint: Pubkey }, // Data is not used.
            #[error("Account not associated with this Mint")]
            MintMismatch (Pubkey), // Data is not used.
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut SetBorshTypesVisitor::new())?;
    korok.accept(&mut SetErrorsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            ProgramNode {
                errors: vec![
                    ErrorNode::new("notRentExempt", 0, "Lamports below rent-exempt threshold"),
                    ErrorNode::new("insufficientFunds", 1, "Insufficient funds"),
                    ErrorNode::new("invalidMint", 2, "Invalid Mint"),
                    ErrorNode::new("mintMismatch", 3, "Account not associated with this Mint"),
                ],
                ..ProgramNode::default()
            }
            .into()
        )
    );
    Ok(())
}

#[test]
fn from_struct() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaErrors)]
        struct MyError {
            mint: Pubkey,
        }
    };
    let mut korok = StructKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut SetBorshTypesVisitor::new())?;
    korok.accept(&mut SetErrorsVisitor::new())?;
    assert_eq!(korok.node, None);
    // No visitor error because there is already is a compilation error.
    Ok(())
}

#[test]
fn no_overrides() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaErrors)]
        enum MyProgramAccounts {}
    };
    let mut korok = EnumKorok::parse(&item)?;
    korok.node = Some(BooleanTypeNode::default().into());

    korok.accept(&mut SetErrorsVisitor::new())?;
    assert_eq!(korok.node, Some(BooleanTypeNode::default().into()));
    Ok(())
}
