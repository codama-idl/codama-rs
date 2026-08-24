use codama_errors::CodamaResult;
use codama_korok_visitors::{IdentifyFieldTypesVisitor, KorokVisitable, SetInstructionsVisitor};
use codama_koroks::{EnumKorok, StructKorok};
use codama_nodes::{
    ArgumentValueNode, InstructionRemainingAccountsNode, IsSigner, NumberFormat::U64,
    NumberTypeNode, PublicKeyTypeNode,
};

#[test]
fn from_struct_with_remaining_accounts_directive() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaInstruction)]
        #[codama(account(name = "source", writable))]
        #[codama(remaining_accounts(
            argument("signers"),
            signer,
            optional,
            docs = "Additional multisig signers."
        ))]
        struct Transfer {
            amount: u64,
        }
    };
    let mut korok = StructKorok::parse(&item)?;

    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut SetInstructionsVisitor::new())?;

    let instruction = match &korok.node {
        Some(codama_nodes::Node::Instruction(instruction)) => instruction,
        _ => panic!("expected InstructionNode"),
    };
    assert_eq!(instruction.accounts.len(), 1);
    assert_eq!(
        instruction.arguments,
        vec![codama_nodes::InstructionArgumentNode::new(
            "amount",
            NumberTypeNode::le(U64)
        )]
    );
    assert_eq!(
        instruction.remaining_accounts,
        vec![InstructionRemainingAccountsNode {
            is_optional: Some(true),
            is_signer: Some(IsSigner::True),
            is_writable: None,
            docs: vec!["Additional multisig signers.".to_string()].into(),
            value: Box::new(ArgumentValueNode::new("signers").into()),
            display: None,
        }]
    );
    Ok(())
}

#[test]
fn from_enum_variant_with_remaining_accounts_directive() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaInstructions)]
        enum MyProgramInstructions {
            #[codama(remaining_accounts(argument("signers"), signer = "either"))]
            Initialize { authority: Pubkey },
            Close,
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut SetInstructionsVisitor::new())?;

    let program = match &korok.node {
        Some(codama_nodes::Node::Program(program)) => program,
        _ => panic!("expected ProgramNode"),
    };
    assert_eq!(
        program.instructions[0].remaining_accounts,
        vec![InstructionRemainingAccountsNode {
            is_optional: None,
            is_signer: Some(IsSigner::Either),
            is_writable: None,
            docs: vec![].into(),
            value: Box::new(ArgumentValueNode::new("signers").into()),
            display: None,
        }]
    );
    // The argument itself is untouched by the directive.
    assert_eq!(
        *program.instructions[0].arguments[1].r#type,
        PublicKeyTypeNode::new().into()
    );
    // Variants without the directive are unaffected.
    assert_eq!(program.instructions[1].remaining_accounts, vec![]);
    Ok(())
}

#[test]
fn multiple_remaining_accounts_directives() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaInstruction)]
        #[codama(remaining_accounts(argument("signers"), signer))]
        #[codama(remaining_accounts(argument("extra_accounts"), writable))]
        struct Execute;
    };
    let mut korok = StructKorok::parse(&item)?;

    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut SetInstructionsVisitor::new())?;

    let instruction = match &korok.node {
        Some(codama_nodes::Node::Instruction(instruction)) => instruction,
        _ => panic!("expected InstructionNode"),
    };
    assert_eq!(
        instruction.remaining_accounts,
        vec![
            InstructionRemainingAccountsNode {
                is_optional: None,
                is_signer: Some(IsSigner::True),
                is_writable: None,
                docs: vec![].into(),
                value: Box::new(ArgumentValueNode::new("signers").into()),
                display: None,
            },
            InstructionRemainingAccountsNode {
                is_optional: None,
                is_signer: None,
                is_writable: Some(true),
                docs: vec![].into(),
                value: Box::new(ArgumentValueNode::new("extraAccounts").into()),
                display: None,
            }
        ]
    );
    Ok(())
}
