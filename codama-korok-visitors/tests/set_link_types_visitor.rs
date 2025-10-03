use codama_errors::CodamaResult;
use codama_korok_visitors::{KorokVisitable, SetLinkTypesVisitor};
use codama_koroks::{FieldKorok, StructKorok};
use codama_nodes::{DefinedTypeLinkNode, NumberFormat::U32, NumberTypeNode, StructFieldTypeNode};

#[test]
fn it_sets_link_nodes_using_the_type_path() -> CodamaResult<()> {
    let item: syn::Field = syn::parse_quote! { Membership };
    let mut korok = FieldKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut SetLinkTypesVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(DefinedTypeLinkNode::new("membership").into())
    );
    Ok(())
}

#[test]
fn it_ignores_the_path_prefix() -> CodamaResult<()> {
    let item: syn::Field = syn::parse_quote! { some::prefix::Membership };
    let mut korok = FieldKorok::parse(&item)?;

    korok.accept(&mut SetLinkTypesVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(DefinedTypeLinkNode::new("membership").into())
    );
    Ok(())
}

#[test]
fn it_ignores_non_path_types() -> CodamaResult<()> {
    let item: syn::Field = syn::parse_quote! { [u32; 8] };
    let mut korok = FieldKorok::parse(&item)?;

    korok.accept(&mut SetLinkTypesVisitor::new())?;
    assert_eq!(korok.node, None);
    Ok(())
}

#[test]
fn it_ignores_types_that_already_have_nodes() -> CodamaResult<()> {
    let item: syn::Field = syn::parse_quote! { u32 };
    let mut korok = FieldKorok::parse(&item)?;
    korok.node = Some(NumberTypeNode::le(U32).into());

    korok.accept(&mut SetLinkTypesVisitor::new())?;
    korok.node = Some(NumberTypeNode::le(U32).into());
    Ok(())
}

#[test]
fn it_works_in_any_parent_koroks() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        pub struct Person(String, u8, Membership);
    };
    let mut korok = StructKorok::parse(&item)?;

    korok.accept(&mut SetLinkTypesVisitor::new())?;
    assert_eq!(
        korok.fields[0].node,
        Some(DefinedTypeLinkNode::new("string").into())
    );
    assert_eq!(
        korok.fields[1].node,
        Some(DefinedTypeLinkNode::new("u8").into())
    );
    assert_eq!(
        korok.fields[2].node,
        Some(DefinedTypeLinkNode::new("membership").into())
    );
    Ok(())
}

#[test]
fn it_create_a_struct_field_type_node_when_nammed() -> CodamaResult<()> {
    let item = syn::parse_quote! { foo: Membership };
    let mut korok = FieldKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut SetLinkTypesVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(StructFieldTypeNode::new("foo", DefinedTypeLinkNode::new("membership")).into())
    );
    Ok(())
}

#[test]
fn it_forwards_the_type_when_unnamed() -> CodamaResult<()> {
    let item = syn::parse_quote! { Membership };
    let mut korok = FieldKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut SetLinkTypesVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(DefinedTypeLinkNode::new("membership").into())
    );
    Ok(())
}
