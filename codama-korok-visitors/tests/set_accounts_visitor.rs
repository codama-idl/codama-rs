use codama_korok_visitors::{
    CombineTypesVisitor, KorokVisitable, SetAccountsVisitor, SetBorshTypesVisitor,
};
use codama_koroks::{EnumKorok, StructKorok};
use codama_nodes::{
    AccountNode, NumberFormat::U64, NumberTypeNode, PublicKeyTypeNode, StructFieldTypeNode,
    StructTypeNode,
};

#[test]
fn it_transforms_defined_type_nodes_into_account_nodes() -> syn::Result<()> {
    let ast: syn::ItemStruct = syn::parse_quote! {
        #[derive(CodamaAccount)]
        struct Token {
            mint: Pubkey,
            owner: Pubkey,
            amount: u64,
        }
    };
    let mut korok = StructKorok::parse(&ast).unwrap();

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
fn it_ignores_enums() -> syn::Result<()> {
    let ast: syn::ItemEnum = syn::parse_quote! {
        #[derive(CodamaAccount)]
        enum Membership {
            Basic,
            Premium,
        }
    };
    let mut korok = EnumKorok::parse(&ast).unwrap();

    assert_eq!(korok.node, None);
    korok.accept(&mut SetBorshTypesVisitor::new())?;
    korok.accept(&mut CombineTypesVisitor::new())?;
    korok.accept(&mut SetAccountsVisitor::new())?;
    assert_eq!(korok.node, None);
    Ok(())
}
