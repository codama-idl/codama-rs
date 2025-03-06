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
            InsufficientFunds,
            #[error("Invalid Mint")]
            InvalidMint,
            #[error("Account not associated with this Mint")]
            MintMismatch,
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

    assert_eq!(korok.node, None);
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
fn from_enum_with_thiserror_prefix() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaErrors)]
        enum MyProgramErrors {
            #[thiserror::error("Lamports below rent-exempt threshold")]
            NotRentExempt,
            #[thiserror::error("Insufficient funds")]
            InsufficientFunds,
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut SetErrorsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            ProgramNode {
                errors: vec![
                    ErrorNode::new("notRentExempt", 0, "Lamports below rent-exempt threshold"),
                    ErrorNode::new("insufficientFunds", 1, "Insufficient funds"),
                ],
                ..ProgramNode::default()
            }
            .into()
        )
    );
    Ok(())
}

#[test]
fn from_enum_with_ignored_data() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaErrors)]
        enum MyProgramErrors {
            #[error("Empty struct")]
            EmptyStruct {},
            #[error("Empty tuple")]
            EmptyTuple (),
            #[error("Filled struct")]
            FilledStruct { mint: Pubkey },
            #[error("Filled tuple")]
            FilledTuple (Pubkey),
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
                    ErrorNode::new("emptyStruct", 0, "Empty struct"),
                    ErrorNode::new("emptyTuple", 1, "Empty tuple"),
                    ErrorNode::new("filledStruct", 2, "Filled struct"),
                    ErrorNode::new("filledTuple", 3, "Filled tuple"),
                ],
                ..ProgramNode::default()
            }
            .into()
        )
    );
    Ok(())
}

#[test]
fn from_enum_with_explicit_discriminators() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaErrors)]
        enum MyProgramErrors {
            #[error("Lamports below rent-exempt threshold")]
            NotRentExempt,
            #[error("Insufficient funds")]
            InsufficientFunds = 42,
            #[error("Invalid Mint")]
            InvalidMint,
            #[error("Account not associated with this Mint")]
            MintMismatch = 100,
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut SetErrorsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            ProgramNode {
                errors: vec![
                    ErrorNode::new("notRentExempt", 0, "Lamports below rent-exempt threshold"),
                    ErrorNode::new("insufficientFunds", 42, "Insufficient funds"),
                    ErrorNode::new("invalidMint", 43, "Invalid Mint"),
                    ErrorNode::new("mintMismatch", 100, "Account not associated with this Mint"),
                ],
                ..ProgramNode::default()
            }
            .into()
        )
    );
    Ok(())
}

#[test]
fn from_enum_with_codama_error_directives() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaErrors)]
        enum MyProgramErrors {
            #[codama(error(600, "Lamports below rent-exempt threshold"))]
            NotRentExempt,
            #[codama(error(601, "Insufficient funds"))]
            InsufficientFunds,
            #[codama(error(602, "Invalid Mint"))]
            InvalidMint,
            #[codama(error(603, "Account not associated with this Mint"))]
            MintMismatch,
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut SetErrorsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            ProgramNode {
                errors: vec![
                    ErrorNode::new("notRentExempt", 600, "Lamports below rent-exempt threshold"),
                    ErrorNode::new("insufficientFunds", 601, "Insufficient funds"),
                    ErrorNode::new("invalidMint", 602, "Invalid Mint"),
                    ErrorNode::new("mintMismatch", 603, "Account not associated with this Mint"),
                ],
                ..ProgramNode::default()
            }
            .into()
        )
    );
    Ok(())
}

#[test]
fn from_enum_with_labeled_codama_error_directives() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaErrors)]
        enum MyProgramErrors {
            #[codama(error(code = 600, message = "Lamports below rent-exempt threshold"))]
            NotRentExempt,
            #[codama(error(code = 601, message = "Insufficient funds"))]
            InsufficientFunds,
            #[codama(error(code = 602, message = "Invalid Mint"))]
            InvalidMint,
            #[codama(error(code = 603, message = "Account not associated with this Mint"))]
            MintMismatch,
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut SetErrorsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            ProgramNode {
                errors: vec![
                    ErrorNode::new("notRentExempt", 600, "Lamports below rent-exempt threshold"),
                    ErrorNode::new("insufficientFunds", 601, "Insufficient funds"),
                    ErrorNode::new("invalidMint", 602, "Invalid Mint"),
                    ErrorNode::new("mintMismatch", 603, "Account not associated with this Mint"),
                ],
                ..ProgramNode::default()
            }
            .into()
        )
    );
    Ok(())
}

#[test]
fn from_enum_with_codama_error_directives_overriding_default_codes() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaErrors)]
        enum MyProgramErrors {
            #[codama(error(10, "Lamports below rent-exempt threshold"))]
            NotRentExempt,
            #[codama(error("Insufficient funds"))]
            InsufficientFunds, // This is 1 and not 10 because #[codama(error(code = x))] does not affect the enum discriminant.
            #[codama(error(50, "Invalid Mint"))]
            InvalidMint = 42,
            #[codama(error("Account not associated with this Mint"))]
            MintMismatch, // This is 43 because the previous variant has a discriminant and no #[codama(error(code = x))] override is provided.
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut SetErrorsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            ProgramNode {
                errors: vec![
                    ErrorNode::new("notRentExempt", 10, "Lamports below rent-exempt threshold"),
                    ErrorNode::new("insufficientFunds", 1, "Insufficient funds"),
                    ErrorNode::new("invalidMint", 50, "Invalid Mint"),
                    ErrorNode::new("mintMismatch", 43, "Account not associated with this Mint"),
                ],
                ..ProgramNode::default()
            }
            .into()
        )
    );
    Ok(())
}

#[test]
fn from_enum_with_codama_error_directives_overriding_thiserror_messages() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaErrors)]
        enum MyProgramErrors {
            #[codama(error("Lamports below rent-exempt threshold"))]
            #[error("This message is overridden")]
            NotRentExempt,
            #[error("This message is overridden")]
            #[codama(error("Insufficient funds"))]
            InsufficientFunds,
            #[error("This message is used")]
            InvalidMint
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut SetErrorsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            ProgramNode {
                errors: vec![
                    ErrorNode::new("notRentExempt", 0, "Lamports below rent-exempt threshold"),
                    ErrorNode::new("insufficientFunds", 1, "Insufficient funds"),
                    ErrorNode::new("invalidMint", 2, "This message is used"),
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
