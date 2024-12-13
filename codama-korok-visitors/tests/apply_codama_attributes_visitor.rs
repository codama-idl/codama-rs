use codama_korok_visitors::{ApplyCodamaAttributesVisitor, KorokVisitable};
use codama_koroks::StructKorok;
use codama_nodes::BooleanTypeNode;

#[test]
fn it_applies_the_attributes_to_the_korok_node() {
    let ast: syn::ItemStruct = syn::parse_quote! {
        #[codama(node(boolean_type))]
        pub struct Membership;
    };
    let mut korok = StructKorok::parse(&ast).unwrap();

    assert_eq!(korok.node, None);
    korok.accept(&mut ApplyCodamaAttributesVisitor::new());
    assert_eq!(korok.node, Some(BooleanTypeNode::default().into()));
}
