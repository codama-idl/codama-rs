use codama_korok_visitors::{KorokVisitable, KorokVisitor, MapVisitor};
use codama_koroks::{KorokMut, KorokTrait, StructKorok};
use codama_nodes::PublicKeyTypeNode;

#[test]
fn it_can_set_a_node_on_all_koroks() {
    let ast: syn::ItemStruct = syn::parse_quote! { struct Foo(u32); };
    let mut korok = StructKorok::parse(&ast).unwrap();

    korok.accept(&mut MapVisitor::new(|mut k, visitor| {
        visitor.visit_children(&mut k);
        k.set_node(Some(PublicKeyTypeNode::new().into()))
    }));

    assert_eq!(korok.node, Some(PublicKeyTypeNode::new().into()));
    assert_eq!(korok.fields.node, Some(PublicKeyTypeNode::new().into()));
    let field = &korok.fields.all[0];
    assert_eq!(field.node, Some(PublicKeyTypeNode::new().into()));
    assert_eq!(field.r#type.node, Some(PublicKeyTypeNode::new().into()));
}

#[test]
fn it_can_reset_all_nodes() {
    let ast: syn::ItemStruct = syn::parse_quote! { struct Foo(u32); };
    let mut korok = StructKorok::parse(&ast).unwrap();
    korok.node = Some(PublicKeyTypeNode::new().into());
    korok.fields.node = Some(PublicKeyTypeNode::new().into());
    let field = &mut korok.fields.all[0];
    field.node = Some(PublicKeyTypeNode::new().into());
    field.r#type.node = Some(PublicKeyTypeNode::new().into());

    korok.accept(&mut MapVisitor::new(|mut k, visitor| {
        visitor.visit_children(&mut k);
        k.set_node(None)
    }));

    assert_eq!(korok.node, None);
    assert_eq!(korok.fields.node, None);
    let field = &korok.fields.all[0];
    assert_eq!(field.node, None);
    assert_eq!(field.r#type.node, None);
}

#[test]
fn is_can_make_decisions_based_on_the_korok_type() {
    let ast: syn::ItemStruct = syn::parse_quote! { struct Foo(u32); };
    let mut korok = StructKorok::parse(&ast).unwrap();

    korok.accept(&mut MapVisitor::new(|mut k, visitor| {
        visitor.visit_children(&mut k);
        match k {
            KorokMut::Type(_) => {
                k.set_node(Some(PublicKeyTypeNode::new().into()));
            }
            _ => {}
        }
    }));

    assert_eq!(korok.node, None);
    assert_eq!(korok.fields.node, None);
    let field = &korok.fields.all[0];
    assert_eq!(field.node, None);
    assert_eq!(field.r#type.node, Some(PublicKeyTypeNode::new().into()));
}
