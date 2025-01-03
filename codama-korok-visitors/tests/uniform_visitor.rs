use codama_errors::CodamaResult;
use codama_korok_visitors::{KorokVisitable, KorokVisitor, UniformVisitor};
use codama_koroks::{KorokMut, KorokTrait, StructKorok};
use codama_nodes::PublicKeyTypeNode;

#[test]
fn it_can_set_a_node_on_all_koroks() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! { struct Foo(u32); };
    let mut korok = StructKorok::parse(&item)?;

    korok.accept(&mut UniformVisitor::new(|mut k, visitor| {
        visitor.visit_children(&mut k)?;
        k.set_node(Some(PublicKeyTypeNode::new().into()));
        Ok(())
    }))?;

    assert_eq!(korok.node, Some(PublicKeyTypeNode::new().into()));
    assert_eq!(korok.fields.node, Some(PublicKeyTypeNode::new().into()));
    let field = &korok.fields.all[0];
    assert_eq!(field.node, Some(PublicKeyTypeNode::new().into()));
    Ok(())
}

#[test]
fn it_can_reset_all_nodes() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! { struct Foo(u32); };
    let mut korok = StructKorok::parse(&item)?;
    korok.node = Some(PublicKeyTypeNode::new().into());
    korok.fields.node = Some(PublicKeyTypeNode::new().into());
    let field = &mut korok.fields.all[0];
    field.node = Some(PublicKeyTypeNode::new().into());

    korok.accept(&mut UniformVisitor::new(|mut k, visitor| {
        visitor.visit_children(&mut k)?;
        k.set_node(None);
        Ok(())
    }))?;

    assert_eq!(korok.node, None);
    assert_eq!(korok.fields.node, None);
    let field = &korok.fields.all[0];
    assert_eq!(field.node, None);
    Ok(())
}

#[test]
fn is_can_make_decisions_based_on_the_korok_type() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! { struct Foo(u32); };
    let mut korok = StructKorok::parse(&item)?;

    korok.accept(&mut UniformVisitor::new(|mut k, visitor| {
        visitor.visit_children(&mut k)?;
        match k {
            KorokMut::Field(_) => {
                k.set_node(Some(PublicKeyTypeNode::new().into()));
            }
            _ => {}
        };
        Ok(())
    }))?;

    assert_eq!(korok.node, None);
    assert_eq!(korok.fields.node, None);
    let field = &korok.fields.all[0];
    assert_eq!(field.node, Some(PublicKeyTypeNode::new().into()));
    Ok(())
}
