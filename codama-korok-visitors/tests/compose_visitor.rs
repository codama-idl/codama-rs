use codama_errors::CodamaResult;
use codama_korok_visitors::{
    walk_korok_mut, walk_struct, ComposeVisitor, KorokVisitable, KorokVisitor, UniformVisitor,
};
use codama_koroks::{ConstKorok, ItemKorok, KorokTrait, StructKorok};
use codama_nodes::PublicKeyTypeNode;

#[test]
fn it_returns_a_single_visitor_from_multiple_visitors() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! { struct Foo(u32); };
    let mut korok = StructKorok::parse(&item)?;

    struct ResetStructKoroksVisitor;
    impl KorokVisitor for ResetStructKoroksVisitor {
        fn visit_struct(&mut self, korok: &mut StructKorok) -> CodamaResult<()> {
            walk_struct(self, korok)?;
            korok.node = None;
            Ok(())
        }
    }

    korok.accept(
        &mut ComposeVisitor::new()
            .with(UniformVisitor::new(|mut k, visitor| {
                walk_korok_mut(visitor, &mut k)?;
                k.set_node(Some(PublicKeyTypeNode::new().into()));
                Ok(())
            }))
            .with(ResetStructKoroksVisitor {}),
    )?;

    assert_eq!(korok.node, None);
    assert_eq!(korok.fields[0].node, Some(PublicKeyTypeNode::new().into()));
    Ok(())
}

#[test]
fn it_forwards_every_kind_of_korok_to_its_visitors() -> CodamaResult<()> {
    // Constants and impl blocks, which the composition used to walk itself without forwarding.
    let item: syn::Item = syn::parse_quote! {
        impl Foo {
            const MAX: u8 = 42;
        }
    };
    let mut korok = ItemKorok::parse(&item, &[], &mut 0)?;

    struct MarkConstsVisitor;
    impl KorokVisitor for MarkConstsVisitor {
        fn visit_const(&mut self, korok: &mut ConstKorok) -> CodamaResult<()> {
            korok.node = Some(PublicKeyTypeNode::new().into());
            Ok(())
        }
    }

    // Entering through the impl block exercises the composition's own `visit_impl`, which did not
    // exist: through the item, each visitor would walk down to the constant on its own.
    let ItemKorok::Impl(r#impl) = &mut korok else {
        panic!("Expected an impl korok");
    };
    r#impl.accept(&mut ComposeVisitor::new().with(MarkConstsVisitor))?;
    assert_eq!(
        r#impl.items[0].node(),
        &Some(PublicKeyTypeNode::new().into())
    );
    Ok(())
}
