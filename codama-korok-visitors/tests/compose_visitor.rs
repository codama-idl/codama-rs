use codama_korok_visitors::{ComposeVisitor, KorokVisitable, KorokVisitor, UniformVisitor};
use codama_koroks::{FieldKorok, KorokTrait, StructKorok};
use codama_nodes::PublicKeyTypeNode;

#[test]
fn it_returns_a_single_visitor_from_multiple_visitors() -> syn::Result<()> {
    let ast: syn::ItemStruct = syn::parse_quote! { struct Foo(u32); };
    let mut korok = StructKorok::parse(&ast).unwrap();

    struct ResetStructAndFieldKoroksVisitor;
    impl KorokVisitor for ResetStructAndFieldKoroksVisitor {
        fn visit_struct(&mut self, korok: &mut StructKorok) -> syn::Result<()> {
            self.visit_children(korok)?;
            korok.node = None;
            Ok(())
        }
        fn visit_field(&mut self, korok: &mut FieldKorok) -> syn::Result<()> {
            self.visit_children(korok)?;
            korok.node = None;
            Ok(())
        }
    }

    korok.accept(
        &mut ComposeVisitor::new()
            .add(UniformVisitor::new(|mut k, visitor| {
                visitor.visit_children(&mut k)?;
                k.set_node(Some(PublicKeyTypeNode::new().into()));
                Ok(())
            }))
            .add(ResetStructAndFieldKoroksVisitor {}),
    )?;

    assert_eq!(korok.node, None);
    assert_eq!(korok.fields.node, Some(PublicKeyTypeNode::new().into()));
    assert_eq!(korok.fields.all[0].node, None);
    Ok(())
}
