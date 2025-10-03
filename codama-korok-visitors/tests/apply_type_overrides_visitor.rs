use codama_errors::CodamaResult;
use codama_korok_visitors::{ApplyTypeOverridesVisitor, KorokVisitable};
use codama_koroks::{FieldKorok, StructKorok};
use codama_nodes::{BooleanTypeNode, StructFieldTypeNode};

#[test]
fn it_set_the_node_on_the_korok() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[codama(type = boolean)]
        pub struct Membership;
    };
    let mut korok = StructKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut ApplyTypeOverridesVisitor::new())?;
    assert_eq!(korok.node, Some(BooleanTypeNode::default().into()));
    Ok(())
}

#[test]
fn it_wraps_the_node_in_a_struct_field_for_named_field_koroks() -> CodamaResult<()> {
    let item: syn::Field = syn::parse_quote! {
        #[codama(type = boolean)]
        pub is_valid: u8
    };
    let mut korok = FieldKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut ApplyTypeOverridesVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(StructFieldTypeNode::new("isValid", BooleanTypeNode::default()).into())
    );
    Ok(())
}

#[test]
fn it_uses_the_name_directive() -> CodamaResult<()> {
    let item: syn::Field = syn::parse_quote! {
        #[codama(name = "is_valid")]
        #[codama(type = boolean)]
        pub valid_mask: u8
    };
    let mut korok = FieldKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut ApplyTypeOverridesVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(StructFieldTypeNode::new("isValid", BooleanTypeNode::default()).into())
    );
    Ok(())
}
