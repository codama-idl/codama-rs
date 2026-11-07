use codama_errors::CodamaResult;
use codama_korok_visitors::{IdentifyFieldTypesVisitor, KorokVisitable, SetAccountsVisitor};
use codama_koroks::{EnumKorok, StructKorok};
use codama_nodes::test_utils::{id, path};
use codama_nodes::{
    AccountNode, BooleanTypeNode, ConstantPdaSeedNode, FieldDiscriminatorNode,
    IntegerFormat::{U64, U8},
    IntegerTypeNode, PdaLinkNode, PdaNode, ProgramNode, PublicKeyTypeNode, SizeDiscriminatorNode,
    StringTypeNode, StringValueNode, StructFieldTypeNode, StructTypeNode, VariablePdaSeedNode,
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
                id("token"),
                StructTypeNode::new(vec![
                    StructFieldTypeNode::new(id("mint"), PublicKeyTypeNode::new()),
                    StructFieldTypeNode::new(id("owner"), PublicKeyTypeNode::new()),
                    StructFieldTypeNode::new(id("amount"), IntegerTypeNode::le(U64)),
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
        Some(AccountNode::new(id("token"), StructTypeNode::new(vec![])).into())
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
        #[codama(identifier = "token")]
        struct TokenAccount;
    };
    let mut korok = StructKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut SetAccountsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(AccountNode::new(id("token"), StructTypeNode::new(vec![])).into())
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
                    FieldDiscriminatorNode::new(path("discriminator"), 0).into(),
                ],
                ..AccountNode::new(id("my_account"), StructTypeNode::new(vec![]))
            }
            .into()
        )
    );
    Ok(())
}

#[test]
fn with_seed_directives() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaAccount)]
        #[codama(seed(type = string(utf8), value = "counter_pda"))]
        #[codama(seed(identifier = "authority"))]
        #[codama(seed(identifier = "identifier", type = integer(u8)))]
        struct Counter {
            authority: Pubkey,
        }
    };
    let mut korok = StructKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut SetAccountsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            ProgramNode {
                accounts: vec![AccountNode {
                    pda: Some(PdaLinkNode::new(id("counter"))),
                    ..AccountNode::new(
                        id("counter"),
                        StructTypeNode::new(vec![StructFieldTypeNode::new(
                            id("authority"),
                            PublicKeyTypeNode::new()
                        ),])
                    )
                }],
                pdas: vec![PdaNode::new(
                    id("counter"),
                    vec![
                        ConstantPdaSeedNode::new(
                            StringTypeNode::utf8(),
                            StringValueNode::new("counter_pda")
                        )
                        .into(),
                        VariablePdaSeedNode::new(id("authority"), PublicKeyTypeNode::new()).into(),
                        VariablePdaSeedNode::new(id("identifier"), IntegerTypeNode::le(U8)).into(),
                    ]
                )],
                ..ProgramNode::default()
            }
            .into()
        )
    );
    Ok(())
}

#[test]
fn with_pda_directive() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaAccount)]
        #[codama(pda = "my_counter_pda")]
        struct Counter;
    };
    let mut korok = StructKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut SetAccountsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            AccountNode {
                pda: Some(PdaLinkNode::new(id("my_counter_pda"))),
                ..AccountNode::new(id("counter"), StructTypeNode::default())
            }
            .into()
        )
    );
    Ok(())
}

#[test]
fn with_pda_and_seed_directives() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaAccount)]
        #[codama(pda = "my_counter_pda")]
        #[codama(seed(identifier = "authority"))]
        struct Counter {
            authority: Pubkey,
        }
    };
    let mut korok = StructKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut SetAccountsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            ProgramNode {
                accounts: vec![AccountNode {
                    pda: Some(PdaLinkNode::new(id("my_counter_pda"))),
                    ..AccountNode::new(
                        id("counter"),
                        StructTypeNode::new(vec![StructFieldTypeNode::new(
                            id("authority"),
                            PublicKeyTypeNode::new()
                        ),])
                    )
                }],
                pdas: vec![PdaNode::new(
                    id("my_counter_pda"),
                    vec![
                        VariablePdaSeedNode::new(id("authority"), PublicKeyTypeNode::new()).into(),
                    ]
                )],
                ..ProgramNode::default()
            }
            .into()
        )
    );
    Ok(())
}
