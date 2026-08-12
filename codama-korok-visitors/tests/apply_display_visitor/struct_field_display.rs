use codama_errors::CodamaResult;
use codama_korok_visitors::{ApplyDisplayVisitor, IdentifyFieldTypesVisitor, KorokVisitable};
use codama_koroks::{FieldKorok, KorokTrait};
use codama_nodes::{
    DisplaySkip, NumberFormat::U64, NumberTypeNode, StringValueNode, StructFieldDisplayNode,
    StructFieldTypeNode,
};

#[test]
fn it_applies_struct_field_displays() -> CodamaResult<()> {
    let ast: syn::Field = syn::parse_quote! {
        #[codama(display(
            label = "Amount",
            skip = when_injected,
            flatten,
            flatten_prefix = "Inner "
        ))]
        amount: u64
    };
    let mut korok = FieldKorok::parse(&ast)?;

    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut ApplyDisplayVisitor::new())?;

    assert_eq!(
        korok.node,
        Some(
            StructFieldTypeNode {
                display: Some(StructFieldDisplayNode {
                    label: Some("Amount".to_string()),
                    skip: Some(DisplaySkip::WhenInjected),
                    flatten: Some(true),
                    flatten_prefix: Some("Inner ".to_string()),
                }),
                ..StructFieldTypeNode::new("amount", NumberTypeNode::le(U64))
            }
            .into()
        )
    );
    Ok(())
}

#[test]
fn it_overlays_multiple_displays_in_source_order() -> CodamaResult<()> {
    let ast: syn::Field = syn::parse_quote! {
        #[codama(display(label = "First", skip = always, flatten))]
        #[codama(display(label = "Second", flatten = false))]
        amount: u64
    };
    let mut korok = FieldKorok::parse(&ast)?;

    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut ApplyDisplayVisitor::new())?;

    assert_eq!(
        korok.node,
        Some(
            StructFieldTypeNode {
                display: Some(StructFieldDisplayNode {
                    label: Some("Second".to_string()),
                    skip: Some(DisplaySkip::Always),
                    flatten: Some(false),
                    flatten_prefix: None,
                }),
                ..StructFieldTypeNode::new("amount", NumberTypeNode::le(U64))
            }
            .into()
        )
    );
    Ok(())
}

#[test]
fn it_ignores_struct_field_properties_on_unnamed_fields() -> CodamaResult<()> {
    let ast: syn::Field = syn::parse_quote! {
        #[codama(display(label = "Amount"))]
        u64
    };
    let mut korok = FieldKorok::parse(&ast)?;

    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut ApplyDisplayVisitor::new())?;

    assert_eq!(korok.node, Some(NumberTypeNode::le(U64).into()));
    Ok(())
}

#[test]
fn it_fails_on_empty_nodes() -> CodamaResult<()> {
    let ast: syn::Field = syn::parse_quote! {
        #[codama(display(label = "Amount"))]
        Unknown
    };
    let mut korok = FieldKorok::parse(&ast)?;

    let error = korok.accept(&mut ApplyDisplayVisitor::new()).unwrap_err();
    assert_eq!(
        error.to_string(),
        "Cannot apply attribute `#[codama(display)]` on an empty node"
    );
    Ok(())
}

#[test]
fn it_fails_to_apply_number_displays_on_non_type_nodes() -> CodamaResult<()> {
    let ast: syn::Field = syn::parse_quote! {
        #[codama(display(amount))]
        Invalid
    };
    let mut korok = FieldKorok::parse(&ast)?;
    korok.set_node(Some(StringValueNode::new("value").into()));

    let error = korok.accept(&mut ApplyDisplayVisitor::new()).unwrap_err();
    assert_eq!(
        error.to_string(),
        "Cannot apply attribute `#[codama(display)]` as a number display on a node of kind `stringValueNode`"
    );
    Ok(())
}
