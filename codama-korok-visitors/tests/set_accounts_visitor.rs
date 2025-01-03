use codama_errors::CodamaResult;
use codama_korok_visitors::{KorokVisitable, SetAccountsVisitor, SetBorshTypesVisitor};
use codama_koroks::{EnumKorok, StructKorok};
use codama_nodes::{
    AccountNode, DefinedTypeNode, FixedSizeTypeNode, NumberFormat::U64, NumberTypeNode,
    PublicKeyTypeNode, StringValueNode, StructFieldTypeNode, StructTypeNode,
};

#[test]
fn it_sets_an_account_node_from_a_struct() -> CodamaResult<()> {
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
    korok.accept(&mut SetBorshTypesVisitor::new())?;
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
fn it_fails_if_the_struct_node_is_not_a_type_node() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaAccount)]
        struct Token;
    };
    let mut korok = StructKorok::parse(&item)?;
    korok.node = Some(StringValueNode::new("Not a `DefinedTypeNode`").into());

    let error = korok.accept(&mut SetAccountsVisitor::new()).unwrap_err();
    assert_eq!(
        error.to_string(),
        "The \"Token\" struct could not be used as an Account because its type is not defined."
    );
    Ok(())
}

#[test]
fn it_fails_if_struct_node_is_not_a_nested_struct_type_node() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaAccount)]
        struct Token;
    };
    let mut korok = StructKorok::parse(&item)?;
    korok.node = Some(
        DefinedTypeNode::new("token", FixedSizeTypeNode::new(NumberTypeNode::le(U64), 42)).into(),
    );

    let error = korok.accept(&mut SetAccountsVisitor::new()).unwrap_err();
    assert_eq!(
        error.to_string(),
        "The \"Token\" struct could not be used as an Account because its type is not a `NestedTypeNode<StructTypeNode>`."
    );
    Ok(())
}

#[test]
fn it_throws_an_error_on_enum_koroks() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaAccount)]
        enum Membership {
            Basic,
            Premium,
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut SetBorshTypesVisitor::new())?;
    let error = korok.accept(&mut SetAccountsVisitor::new()).unwrap_err();
    assert_eq!(
        error.to_string(),"The \"Membership\" enum could not be used as an Account because only structs are currently accepted."
    );
    Ok(())
}
