use codama_errors::CodamaResult;
use codama_korok_visitors::{KorokVisitable, SetBorshTypesVisitor, SetInstructionsVisitor};
use codama_koroks::{EnumKorok, StructKorok};
use codama_nodes::{
    BooleanTypeNode, InstructionAccountNode, InstructionArgumentNode, InstructionNode,
    NumberFormat::U64, NumberTypeNode,
};

#[test]
fn from_struct() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaInstruction)]
        struct Initialize {
            #[codama(account)]
            authority: AccountMeta,
            #[codama(account(signer, writable))]
            payer: AccountMeta,
            amount: u64,
            is_canonical: bool,
        }
    };
    let mut korok = StructKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut SetBorshTypesVisitor::new())?;
    korok.accept(&mut SetInstructionsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            InstructionNode {
                name: "initialize".into(),
                accounts: vec![
                    InstructionAccountNode::new("authority", false, false),
                    InstructionAccountNode::new("payer", true, true),
                ],
                arguments: vec![
                    InstructionArgumentNode::new("amount", NumberTypeNode::le(U64)),
                    InstructionArgumentNode::new("is_canonical", BooleanTypeNode::default(),),
                ],
                ..InstructionNode::default()
            }
            .into()
        )
    );
    Ok(())
}

#[test]
fn from_struct_with_arguments_only() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaInstruction)]
        struct Initialize {
            amount: u64,
            is_canonical: bool,
        }
    };
    let mut korok = StructKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut SetBorshTypesVisitor::new())?;
    korok.accept(&mut SetInstructionsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            InstructionNode {
                name: "initialize".into(),
                arguments: vec![
                    InstructionArgumentNode::new("amount", NumberTypeNode::le(U64)),
                    InstructionArgumentNode::new("is_canonical", BooleanTypeNode::default(),),
                ],
                ..InstructionNode::default()
            }
            .into()
        )
    );
    Ok(())
}

#[test]
fn from_struct_with_accounts_only() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaInstruction)]
        struct Initialize {
            #[codama(account)]
            authority: AccountMeta,
            #[codama(account(signer, writable))]
            payer: AccountMeta,
        }
    };
    let mut korok = StructKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut SetBorshTypesVisitor::new())?;
    korok.accept(&mut SetInstructionsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            InstructionNode {
                name: "initialize".into(),
                accounts: vec![
                    InstructionAccountNode::new("authority", false, false),
                    InstructionAccountNode::new("payer", true, true),
                ],
                ..InstructionNode::default()
            }
            .into()
        )
    );
    Ok(())
}

#[test]
fn from_struct_with_accounts_as_struct_attributes() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaInstruction)]
        #[codama(account(name = "authority"))]
        #[codama(account(name = "payer", signer, writable))]
        struct Initialize {
            amount: u64,
            is_canonical: bool,
        }
    };
    let mut korok = StructKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut SetBorshTypesVisitor::new())?;
    korok.accept(&mut SetInstructionsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            InstructionNode {
                name: "initialize".into(),
                accounts: vec![
                    InstructionAccountNode::new("authority", false, false),
                    InstructionAccountNode::new("payer", true, true),
                ],
                arguments: vec![
                    InstructionArgumentNode::new("amount", NumberTypeNode::le(U64)),
                    InstructionArgumentNode::new("is_canonical", BooleanTypeNode::default(),),
                ],
                ..InstructionNode::default()
            }
            .into()
        )
    );
    Ok(())
}

#[test]
fn from_empty_struct() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaInstruction)]
        struct Initialize;
    };
    let mut korok = StructKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut SetBorshTypesVisitor::new())?;
    korok.accept(&mut SetInstructionsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            InstructionNode {
                name: "initialize".into(),
                ..InstructionNode::default()
            }
            .into()
        )
    );
    Ok(())
}

#[test]
fn from_enum() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaInstruction)]
        enum MyInstructions {
            Initialize,
            Update,
            Close,
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut SetBorshTypesVisitor::new())?;
    korok.accept(&mut SetInstructionsVisitor::new())?;
    assert_eq!(korok.node, None);
    // No visitor error because there is already is a compilation error.
    Ok(())
}

#[test]
fn no_overrides() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaInstruction)]
        struct Initialize;
    };
    let mut korok = StructKorok::parse(&item)?;
    korok.node = Some(BooleanTypeNode::default().into());

    korok.accept(&mut SetInstructionsVisitor::new())?;
    assert_eq!(korok.node, Some(BooleanTypeNode::default().into()));
    Ok(())
}
