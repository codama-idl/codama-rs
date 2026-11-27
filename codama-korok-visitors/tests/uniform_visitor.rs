use codama_errors::CodamaResult;
use codama_korok_visitors::{walk_korok_mut, KorokVisitable, UniformVisitor};
use codama_koroks::{ItemKorok, KorokMut, KorokTrait, StructKorok};
use codama_nodes::PublicKeyTypeNode;

#[test]
fn it_can_set_a_node_on_all_koroks() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! { struct Foo(u32); };
    let mut korok = StructKorok::parse(&item)?;

    korok.accept(&mut UniformVisitor::new(|mut k, visitor| {
        walk_korok_mut(visitor, &mut k)?;
        k.set_node(Some(PublicKeyTypeNode::new().into()));
        Ok(())
    }))?;

    assert_eq!(korok.node, Some(PublicKeyTypeNode::new().into()));
    assert_eq!(korok.fields[0].node, Some(PublicKeyTypeNode::new().into()));
    Ok(())
}

#[test]
fn it_can_reset_all_nodes() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! { struct Foo(u32); };
    let mut korok = StructKorok::parse(&item)?;
    korok.node = Some(PublicKeyTypeNode::new().into());
    korok.fields[0].node = Some(PublicKeyTypeNode::new().into());

    korok.accept(&mut UniformVisitor::new(|mut k, visitor| {
        walk_korok_mut(visitor, &mut k)?;
        k.set_node(None);
        Ok(())
    }))?;

    assert_eq!(korok.node, None);
    assert_eq!(korok.fields[0].node, None);
    Ok(())
}

#[test]
fn is_can_make_decisions_based_on_the_korok_type() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! { struct Foo(u32); };
    let mut korok = StructKorok::parse(&item)?;

    korok.accept(&mut UniformVisitor::new(|mut k, visitor| {
        walk_korok_mut(visitor, &mut k)?;
        if let KorokMut::Field(_) = k {
            k.set_node(Some(PublicKeyTypeNode::new().into()));
        };
        Ok(())
    }))?;

    assert_eq!(korok.node, None);
    assert_eq!(korok.fields[0].node, Some(PublicKeyTypeNode::new().into()));
    Ok(())
}

#[test]
fn it_visits_impl_blocks() -> CodamaResult<()> {
    // Impl blocks were missing from `KorokMut`, so uniform visitors skipped them and their items.
    let item: syn::Item = syn::parse_quote! {
        impl Foo {
            const MAX: u8 = 42;
        }
    };
    let mut korok = ItemKorok::parse(&item, &[], &mut 0)?;
    korok.accept(&mut UniformVisitor::new(|mut k, visitor| {
        walk_korok_mut(visitor, &mut k)?;
        k.set_node(Some(PublicKeyTypeNode::new().into()));
        Ok(())
    }))?;
    let ItemKorok::Impl(r#impl) = &korok else {
        panic!("Expected an impl korok");
    };
    assert_eq!(r#impl.node, Some(PublicKeyTypeNode::new().into()));
    assert_eq!(
        r#impl.items[0].node(),
        &Some(PublicKeyTypeNode::new().into())
    );
    Ok(())
}
