use codama_korok_visitors::{KorokVisitable, SetBorshTypesVisitor};
use codama_koroks::FieldKorok;
use codama_nodes::BooleanTypeNode;

#[test]
fn it_does_not_override_existing_nodes() -> syn::Result<()> {
    let ast: syn::Field = syn::parse_quote! { u32 };
    let mut korok = FieldKorok::parse(&ast).unwrap();
    korok.node = Some(BooleanTypeNode::default().into());
    korok.accept(&mut SetBorshTypesVisitor::new())?;
    assert_eq!(korok.node, Some(BooleanTypeNode::default().into()));
    Ok(())
}
