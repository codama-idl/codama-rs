use codama_errors::CodamaResult;
use codama_korok_visitors::{ApplyCodamaTypeAttributesVisitor, KorokVisitable};
use codama_koroks::{FieldKorok, StructKorok};
use codama_nodes::{BooleanTypeNode, StructFieldTypeNode};

#[test]
fn it_set_the_node_on_the_korok() -> CodamaResult<()> {
    let ast: syn::ItemStruct = syn::parse_quote! {
        #[codama(type = boolean)]
        pub struct Membership;
    };
    let mut korok = StructKorok::parse(&ast)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut ApplyCodamaTypeAttributesVisitor::new())?;
    assert_eq!(korok.node, Some(BooleanTypeNode::default().into()));
    Ok(())
}

#[test]
fn it_wraps_the_node_in_a_struct_field_for_named_field_koroks() -> CodamaResult<()> {
    let ast: syn::Field = syn::parse_quote! {
        #[codama(type = boolean)]
        pub is_valid: u8
    };
    let mut korok = FieldKorok::parse(&ast)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut ApplyCodamaTypeAttributesVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(StructFieldTypeNode::new("isValid", BooleanTypeNode::default()).into())
    );
    Ok(())
}
