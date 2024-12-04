use codama_korok_visitors::{DefineBorshTypesVisitor, KorokVisitable};
use codama_koroks::TypeKorok;
use codama_nodes::BooleanTypeNode;
use codama_syn_helpers::syn_build;
use quote::quote;

#[test]
fn it_does_not_override_existing_nodes() {
    let ast = syn_build::parse(quote! { u32 });
    let mut korok = TypeKorok::new(&ast);
    korok.node = Some(BooleanTypeNode::default().into());
    korok.accept(&mut DefineBorshTypesVisitor::new());
    assert_eq!(korok.node, Some(BooleanTypeNode::default().into()));
}
