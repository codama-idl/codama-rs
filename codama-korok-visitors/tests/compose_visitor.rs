use codama_korok_visitors::{ComposeVisitor, KorokVisitable, KorokVisitor, UniformVisitor};
use codama_koroks::{FieldKorok, KorokTrait, StructKorok};
use codama_nodes::PublicKeyTypeNode;

#[test]
fn it_returns_a_single_visitor_from_multiple_visitors() {
    let ast: syn::ItemStruct = syn::parse_quote! { struct Foo(u32); };
    let mut korok = StructKorok::parse(&ast).unwrap();

    struct ResetStructAndFieldKoroksVisitor;
    impl KorokVisitor for ResetStructAndFieldKoroksVisitor {
        fn visit_struct(&mut self, korok: &mut StructKorok) {
            self.visit_children(korok);
            korok.node = None;
        }
        fn visit_field(&mut self, korok: &mut FieldKorok) {
            self.visit_children(korok);
            korok.node = None;
        }
    }

    korok.accept(
        &mut ComposeVisitor::new()
            .add(UniformVisitor::new(|mut k, visitor| {
                visitor.visit_children(&mut k);
                k.set_node(Some(PublicKeyTypeNode::new().into()))
            }))
            .add(ResetStructAndFieldKoroksVisitor {}),
    );

    assert_eq!(korok.node, None);
    assert_eq!(korok.fields.node, Some(PublicKeyTypeNode::new().into()));
    assert_eq!(korok.fields.all[0].node, None);
}
