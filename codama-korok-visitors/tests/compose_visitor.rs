use codama_errors::CodamaResult;
use codama_korok_visitors::{ComposeVisitor, KorokVisitable, KorokVisitor, UniformVisitor};
use codama_koroks::{KorokTrait, StructKorok};
use codama_nodes::PublicKeyTypeNode;

#[test]
fn it_returns_a_single_visitor_from_multiple_visitors() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! { struct Foo(u32); };
    let mut korok = StructKorok::parse(&item)?;

    struct ResetStructKoroksVisitor;
    impl KorokVisitor for ResetStructKoroksVisitor {
        fn visit_struct(&mut self, korok: &mut StructKorok) -> CodamaResult<()> {
            self.visit_children(korok)?;
            korok.node = None;
            Ok(())
        }
    }

    korok.accept(
        &mut ComposeVisitor::new()
            .with(UniformVisitor::new(|mut k, visitor| {
                visitor.visit_children(&mut k)?;
                k.set_node(Some(PublicKeyTypeNode::new().into()));
                Ok(())
            }))
            .with(ResetStructKoroksVisitor {}),
    )?;

    assert_eq!(korok.node, None);
    assert_eq!(korok.fields[0].node, Some(PublicKeyTypeNode::new().into()));
    Ok(())
}
