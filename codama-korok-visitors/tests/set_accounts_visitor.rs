use codama_errors::CodamaResult;
use codama_korok_visitors::{
    CombineTypesVisitor, KorokVisitable, SetAccountsVisitor, SetBorshTypesVisitor,
};
use codama_koroks::{EnumKorok, StructKorok};
use codama_nodes::{
    AccountNode, NumberFormat::U64, NumberTypeNode, PublicKeyTypeNode, StructFieldTypeNode,
    StructTypeNode,
};

#[test]
fn it_transforms_defined_type_nodes_into_account_nodes() -> CodamaResult<()> {
    let ast: syn::ItemStruct = syn::parse_quote! {
        #[derive(CodamaAccount)]
        struct Token {
            mint: Pubkey,
            owner: Pubkey,
            amount: u64,
        }
    };
    let mut korok = StructKorok::parse(&ast)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut SetBorshTypesVisitor::new())?;
    korok.accept(&mut CombineTypesVisitor::new())?;
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
fn it_requires_the_account_type_to_be_defined_first() -> CodamaResult<()> {
    let ast: syn::ItemStruct = syn::parse_quote! {
        #[derive(CodamaAccount)]
        struct Token {
            mint: Pubkey,
            owner: Pubkey,
            amount: u64,
        }
    };
    let mut korok = StructKorok::parse(&ast)?;

    assert_eq!(korok.node, None);
    let error = korok.accept(&mut SetAccountsVisitor::new()).unwrap_err();
    assert_eq!(
        error.to_string(),
        "The \"Token\" struct could not be used as an Account because its type is not defined."
    );
    Ok(())
}

#[test]
fn it_throws_an_error_on_enum_koroks() -> CodamaResult<()> {
    let ast: syn::ItemEnum = syn::parse_quote! {
        #[derive(CodamaAccount)]
        enum Membership {
            Basic,
            Premium,
        }
    };
    let mut korok = EnumKorok::parse(&ast)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut SetBorshTypesVisitor::new())?;
    korok.accept(&mut CombineTypesVisitor::new())?;
    let error = korok.accept(&mut SetAccountsVisitor::new()).unwrap_err();
    assert_eq!(
        error.to_string(),"The \"Membership\" enum could not be used as an Account because only structs are currently accepted."
    );
    Ok(())
}
