use codama_errors::CodamaResult;
use codama_korok_visitors::{IdentifyFieldTypesVisitor, KorokVisitable, SetAccountsVisitor};
use codama_koroks::{EnumKorok, StructKorok};
use codama_nodes::{
    AccountNode, BooleanTypeNode, FieldDiscriminatorNode, NumberFormat::U64, NumberTypeNode,
    PublicKeyTypeNode, SizeDiscriminatorNode, StructFieldTypeNode, StructTypeNode,
};

#[test]
fn from_struct() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaAccount)]
        struct Token {
            mint: Pubkey,
            owner: Pubkey,
            amount: u64,
        }
    };
    let mut korok = StructKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut SetAccountsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            AccountNode::new(
                "token",
                StructTypeNode::new(vec![
                    StructFieldTypeNode::new("mint", PublicKeyTypeNode::new()),
                    StructFieldTypeNode::new("owner", PublicKeyTypeNode::new()),
                    StructFieldTypeNode::new("amount", NumberTypeNode::le(U64)),
                ])
            )
            .into()
        )
    );
    Ok(())
}

#[test]
fn from_empty_struct() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaAccount)]
        struct Token;
    };
    let mut korok = StructKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut SetAccountsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(AccountNode::new("token", StructTypeNode::new(vec![])).into())
    );
    Ok(())
}

#[test]
fn from_enum() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaAccount)]
        enum Membership {
            Basic,
            Premium,
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut SetAccountsVisitor::new())?;
    assert_eq!(korok.node, None);
    // No visitor error because there is already is a compilation error.
    Ok(())
}

#[test]
fn no_overrides() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaAccount)]
        struct Token;
    };
    let mut korok = StructKorok::parse(&item)?;
    korok.node = Some(BooleanTypeNode::default().into());

    korok.accept(&mut SetAccountsVisitor::new())?;
    assert_eq!(korok.node, Some(BooleanTypeNode::default().into()));
    Ok(())
}

#[test]
fn with_name_directive() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaAccount)]
        #[codama(name = "token")]
        struct TokenAccount;
    };
    let mut korok = StructKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut SetAccountsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(AccountNode::new("token", StructTypeNode::new(vec![])).into())
    );
    Ok(())
}

#[test]
fn with_discriminator_directives() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaAccount)]
        #[codama(discriminator(size = 100))]
        #[codama(discriminator(field = "discriminator"))]
        struct MyAccount;
    };
    let mut korok = StructKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut SetAccountsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            AccountNode {
                discriminators: vec![
                    SizeDiscriminatorNode::new(100).into(),
                    FieldDiscriminatorNode::new("discriminator", 0).into(),
                ],
                ..AccountNode::new("myAccount", StructTypeNode::new(vec![]))
            }
            .into()
        )
    );
    Ok(())
}
