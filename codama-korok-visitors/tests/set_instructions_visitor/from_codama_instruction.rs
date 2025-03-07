use codama_errors::CodamaResult;
use codama_korok_visitors::{KorokVisitable, SetBorshTypesVisitor, SetInstructionsVisitor};
use codama_koroks::{EnumKorok, StructKorok};
use codama_nodes::{
    BooleanTypeNode, Docs, InstructionAccountNode, InstructionArgumentNode, InstructionNode,
    NumberFormat::U64, NumberTypeNode, PayerValueNode, PublicKeyValueNode,
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
fn from_struct_with_default_values_in_accounts() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaInstruction)]
        #[codama(account(name = "rent_sysvar", default_value = sysvar("rent")))]
        struct Initialize {
            #[codama(account)]
            authority: AccountMeta,
            #[codama(account(signer, writable, default_value = payer))]
            payer: AccountMeta,
        }
    };
    let mut korok = StructKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut SetInstructionsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            InstructionNode {
                name: "initialize".into(),
                accounts: vec![
                    InstructionAccountNode {
                        name: "rentSysvar".into(),
                        default_value: Some(
                            PublicKeyValueNode::new("SysvarRent111111111111111111111111111111111")
                                .into()
                        ),
                        is_writable: false,
                        is_signer: false.into(),
                        is_optional: false,
                        docs: Docs::default(),
                    },
                    InstructionAccountNode::new("authority", false, false),
                    InstructionAccountNode {
                        name: "payer".into(),
                        is_signer: true.into(),
                        is_writable: true,
                        default_value: Some(PayerValueNode::new().into()),
                        is_optional: false,
                        docs: Docs::default(),
                    },
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
