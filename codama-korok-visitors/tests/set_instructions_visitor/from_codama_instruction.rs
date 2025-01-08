use codama_errors::CodamaResult;
use codama_korok_visitors::{KorokVisitable, SetBorshTypesVisitor, SetInstructionsVisitor};
use codama_koroks::{EnumKorok, StructKorok};
use codama_nodes::{
    BooleanTypeNode, DefinedTypeNode, FixedSizeTypeNode, InstructionAccountNode,
    InstructionArgumentNode, InstructionNode, NumberFormat::U64, NumberTypeNode, StringValueNode,
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

// TODO: With accounts as struct attributes

#[test]
fn from_struct_not_a_type_node() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaInstruction)]
        struct Initialize;
    };
    let mut korok = StructKorok::parse(&item)?;
    korok.node = Some(StringValueNode::new("Not a `DefinedTypeNode`").into());

    let error = korok
        .accept(&mut SetInstructionsVisitor::new())
        .unwrap_err();
    assert_eq!(
        error.to_string(),
        "The \"Initialize\" struct could not be used as an Instruction because its type is not defined."
    );
    Ok(())
}

#[test]
fn from_struct_not_a_struct_type_node() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaInstruction)]
        struct Initialize;
    };
    let mut korok = StructKorok::parse(&item)?;
    korok.node = Some(
        DefinedTypeNode::new(
            "initialize",
            FixedSizeTypeNode::new(NumberTypeNode::le(U64), 42),
        )
        .into(),
    );

    let error = korok
        .accept(&mut SetInstructionsVisitor::new())
        .unwrap_err();
    assert_eq!(
        error.to_string(),
        "The \"Initialize\" struct could not be used as an Instruction because its type is not a `StructTypeNode`."
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
    let error = korok
        .accept(&mut SetInstructionsVisitor::new())
        .unwrap_err();
    assert_eq!(
        error.to_string(),"The \"MyInstructions\" enum could not be used as an Instruction because the `CodamaInstruction` derive can only be used on structs. Did you mean to use `CodamaInstructions` instead?"
    );
    Ok(())
}
