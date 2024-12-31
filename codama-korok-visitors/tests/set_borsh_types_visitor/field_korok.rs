use codama_korok_visitors::{KorokVisitable, SetBorshTypesVisitor};
use codama_koroks::FieldKorok;
use codama_nodes::{NumberTypeNode, StructFieldTypeNode, U64};

#[test]
fn it_create_a_struct_field_type_node_when_nammed() -> syn::Result<()> {
    let ast = syn::parse_quote! { foo: u64 };
    let mut korok = FieldKorok::parse(&ast).unwrap();

    assert_eq!(korok.node, None);
    korok.accept(&mut SetBorshTypesVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(StructFieldTypeNode::new("foo", NumberTypeNode::le(U64)).into())
    );
    Ok(())
}

#[test]
fn it_forwards_the_type_when_unnamed() -> syn::Result<()> {
    let ast = syn::parse_quote! { u64 };
    let mut korok = FieldKorok::parse(&ast).unwrap();

    assert_eq!(korok.node, None);
    korok.accept(&mut SetBorshTypesVisitor::new())?;
    assert_eq!(korok.node, Some(NumberTypeNode::le(U64).into()));
    Ok(())
}
