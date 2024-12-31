use codama_korok_visitors::{KorokVisitable, SetLinkTypesVisitor};
use codama_koroks::{FieldKorok, StructKorok};
use codama_nodes::{DefinedTypeLinkNode, NumberFormat::U32, NumberTypeNode, StructFieldTypeNode};

#[test]
fn it_sets_link_nodes_using_the_type_path() -> syn::Result<()> {
    let ast: syn::Field = syn::parse_quote! { Membership };
    let mut korok = FieldKorok::parse(&ast).unwrap();

    assert_eq!(korok.node, None);
    korok.accept(&mut SetLinkTypesVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(DefinedTypeLinkNode::new("membership").into())
    );
    Ok(())
}

#[test]
fn it_ignores_the_path_prefix() -> syn::Result<()> {
    let ast: syn::Field = syn::parse_quote! { some::prefix::Membership };
    let mut korok = FieldKorok::parse(&ast).unwrap();

    korok.accept(&mut SetLinkTypesVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(DefinedTypeLinkNode::new("membership").into())
    );
    Ok(())
}

#[test]
fn it_ignores_non_path_types() -> syn::Result<()> {
    let ast: syn::Field = syn::parse_quote! { [u32; 8] };
    let mut korok = FieldKorok::parse(&ast).unwrap();

    korok.accept(&mut SetLinkTypesVisitor::new())?;
    assert_eq!(korok.node, None);
    Ok(())
}

#[test]
fn it_ignores_types_that_already_have_nodes() -> syn::Result<()> {
    let ast: syn::Field = syn::parse_quote! { u32 };
    let mut korok = FieldKorok::parse(&ast).unwrap();
    korok.node = Some(NumberTypeNode::le(U32).into());

    korok.accept(&mut SetLinkTypesVisitor::new())?;
    korok.node = Some(NumberTypeNode::le(U32).into());
    Ok(())
}

#[test]
fn it_works_in_any_parent_koroks() -> syn::Result<()> {
    let ast: syn::ItemStruct = syn::parse_quote! {
        pub struct Person(String, u8, Membership);
    };
    let mut korok = StructKorok::parse(&ast).unwrap();

    korok.accept(&mut SetLinkTypesVisitor::new())?;
    let fields = korok.fields.all;
    assert_eq!(
        fields[0].node,
        Some(DefinedTypeLinkNode::new("string").into())
    );
    assert_eq!(fields[1].node, Some(DefinedTypeLinkNode::new("u8").into()));
    assert_eq!(
        fields[2].node,
        Some(DefinedTypeLinkNode::new("membership").into())
    );
    Ok(())
}

#[test]
fn it_create_a_struct_field_type_node_when_nammed() -> syn::Result<()> {
    let ast = syn::parse_quote! { foo: Membership };
    let mut korok = FieldKorok::parse(&ast).unwrap();

    assert_eq!(korok.node, None);
    korok.accept(&mut SetLinkTypesVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(StructFieldTypeNode::new("foo", DefinedTypeLinkNode::new("membership")).into())
    );
    Ok(())
}

#[test]
fn it_forwards_the_type_when_unnamed() -> syn::Result<()> {
    let ast = syn::parse_quote! { Membership };
    let mut korok = FieldKorok::parse(&ast).unwrap();

    assert_eq!(korok.node, None);
    korok.accept(&mut SetLinkTypesVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(DefinedTypeLinkNode::new("membership").into())
    );
    Ok(())
}
