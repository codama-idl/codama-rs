use codama_korok_visitors::{ComposeVisitor, KorokVisitable, KorokVisitor, MapVisitor};
use codama_koroks::{KorokTrait, StructKorok, TypeKorok};
use codama_nodes::PublicKeyTypeNode;

#[test]
fn it_returns_a_single_visitor_from_multiple_visitors() {
    let ast: syn::ItemStruct = syn::parse_quote! { struct Foo(u32); };
    let mut korok = StructKorok::parse(&ast).unwrap();

    struct ResetStructAndTypeKoroksVisitor;
    impl KorokVisitor for ResetStructAndTypeKoroksVisitor {
        fn visit_struct(&mut self, korok: &mut StructKorok) {
            self.visit_children(korok);
            korok.node = None;
        }
        fn visit_type(&mut self, korok: &mut TypeKorok) {
            self.visit_children(korok);
            korok.node = None;
        }
    }

    korok.accept(
        &mut ComposeVisitor::new()
            .add(MapVisitor::new(|mut k| {
                k.set_node(Some(PublicKeyTypeNode::new().into()))
            }))
            .add(ResetStructAndTypeKoroksVisitor {}),
    );

    assert_eq!(korok.node, None);
    assert_eq!(korok.fields.node, Some(PublicKeyTypeNode::new().into()));
    let field = &korok.fields.all[0];
    assert_eq!(field.node, Some(PublicKeyTypeNode::new().into()));
    assert_eq!(field.r#type.node, None);
}
