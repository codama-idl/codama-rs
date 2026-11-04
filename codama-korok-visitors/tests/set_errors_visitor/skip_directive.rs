use codama_errors::CodamaResult;
use codama_korok_visitors::{KorokVisitable, SetErrorsVisitor};
use codama_koroks::EnumKorok;
use codama_nodes::test_utils::id;
use codama_nodes::{ErrorNode, ProgramNode};

#[test]
fn skip_variant_in_enum() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaErrors)]
        enum MyProgramErrors {
            #[error("Lamports below rent-exempt threshold")]
            NotRentExempt,
            #[codama(skip)]
            #[error("Internal error")]
            InternalOnly,
            #[error("Insufficient funds")]
            InsufficientFunds,
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

    korok.accept(&mut SetErrorsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            ProgramNode {
                errors: vec![
                    ErrorNode::new(
                        id("not_rent_exempt"),
                        0,
                        "Lamports below rent-exempt threshold"
                    ),
                    ErrorNode::new(id("insufficient_funds"), 2, "Insufficient funds"),
                ],
                ..ProgramNode::default()
            }
            .into()
        )
    );
    Ok(())
}

#[test]
fn skip_variant_with_explicit_discriminator() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaErrors)]
        enum MyProgramErrors {
            #[error("Error A")]
            ErrorA,
            #[error("Error B")]
            ErrorB = 10,
            #[codama(skip)]
            #[error("Internal")]
            InternalOnly = 228,
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

    korok.accept(&mut SetErrorsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            ProgramNode {
                errors: vec![
                    ErrorNode::new(id("error_a"), 0, "Error A"),
                    ErrorNode::new(id("error_b"), 10, "Error B"),
                ],
                ..ProgramNode::default()
            }
            .into()
        )
    );
    Ok(())
}

#[test]
fn skip_preserves_sibling_discriminator_counting() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaErrors)]
        enum MyProgramErrors {
            #[error("First")]
            First,
            #[codama(skip)]
            #[error("Second")]
            Second,
            #[error("Third")]
            Third,
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

    korok.accept(&mut SetErrorsVisitor::new())?;

    // Third should have code 2 (not 1), because
    // Second still occupies slot 1 even though it's skipped.
    assert_eq!(
        korok.node,
        Some(
            ProgramNode {
                errors: vec![
                    ErrorNode::new(id("first"), 0, "First"),
                    ErrorNode::new(id("third"), 2, "Third"),
                ],
                ..ProgramNode::default()
            }
            .into()
        )
    );
    Ok(())
}

#[test]
fn skip_all_variants() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaErrors)]
        enum MyProgramErrors {
            #[codama(skip)]
            #[error("First")]
            First,
            #[codama(skip)]
            #[error("Second")]
            Second,
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

    korok.accept(&mut SetErrorsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            ProgramNode {
                errors: vec![],
                ..ProgramNode::default()
            }
            .into()
        )
    );
    Ok(())
}
