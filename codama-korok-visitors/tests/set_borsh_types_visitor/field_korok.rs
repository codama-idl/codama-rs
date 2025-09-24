use codama_errors::CodamaResult;
use codama_korok_visitors::{KorokVisitable, SetBorshTypesVisitor};
use codama_koroks::FieldKorok;
use codama_nodes::{NumberTypeNode, StructFieldTypeNode, U32, U64};

#[test]
fn it_create_a_struct_field_type_node_when_nammed() -> CodamaResult<()> {
    let ast = syn::parse_quote! { foo: u64 };
    let mut korok = FieldKorok::parse(&ast)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut SetBorshTypesVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(StructFieldTypeNode::new("foo", NumberTypeNode::le(U64)).into())
    );
    Ok(())
}

#[test]
fn it_forwards_the_type_when_unnamed() -> CodamaResult<()> {
    let ast = syn::parse_quote! { u64 };
    let mut korok = FieldKorok::parse(&ast)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut SetBorshTypesVisitor::new())?;
    assert_eq!(korok.node, Some(NumberTypeNode::le(U64).into()));
    Ok(())
}

#[test]
fn it_uses_the_name_directive() -> CodamaResult<()> {
    let ast: syn::Field = syn::parse_quote! {
        #[codama(name = "orange")]
        pub apple: u32
    };
    let mut korok = FieldKorok::parse(&ast)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut SetBorshTypesVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(StructFieldTypeNode::new("orange", NumberTypeNode::le(U32)).into())
    );
    Ok(())
}
