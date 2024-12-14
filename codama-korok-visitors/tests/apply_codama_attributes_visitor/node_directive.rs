use codama_korok_visitors::{ApplyCodamaAttributesVisitor, KorokVisitable};
use codama_koroks::{FieldKorok, StructKorok};
use codama_nodes::{BooleanTypeNode, StructFieldTypeNode};

#[test]
fn it_set_the_node_on_the_korok() {
    let ast: syn::ItemStruct = syn::parse_quote! {
        #[codama(node(boolean_type))]
        pub struct Membership;
    };
    let mut korok = StructKorok::parse(&ast).unwrap();

    assert_eq!(korok.node, None);
    korok.accept(&mut ApplyCodamaAttributesVisitor::new());
    assert_eq!(korok.node, Some(BooleanTypeNode::default().into()));
}

#[test]
fn it_wraps_the_node_in_a_struct_field_for_named_field_koroks() {
    let ast: syn::Field = syn::parse_quote! {
        #[codama(node(boolean_type))]
        pub is_valid: u8
    };
    let mut korok = FieldKorok::parse(&ast).unwrap();

    assert_eq!(korok.node, None);
    korok.accept(&mut ApplyCodamaAttributesVisitor::new());
    assert_eq!(
        korok.node,
        Some(StructFieldTypeNode::new("isValid", BooleanTypeNode::default()).into())
    );
}
