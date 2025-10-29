use codama_errors::CodamaResult;
use codama_korok_visitors::{IdentifyFieldTypesVisitor, KorokVisitable, SetDefaultValuesVisitor};
use codama_koroks::{FieldKorok, StructKorok};
use codama_nodes::{
    InstructionArgumentNode, NumberFormat::U8, NumberTypeNode, NumberValueNode, PayerValueNode,
    PublicKeyTypeNode, StructFieldTypeNode, StructTypeNode,
};

#[test]
fn it_sets_default_values_to_struct_field_type_nodes() -> CodamaResult<()> {
    let item: syn::Field = syn::parse_quote! {
        #[codama(default_value = 42)]
        pub amount: u8
    };
    let mut korok = FieldKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut SetDefaultValuesVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            StructFieldTypeNode {
                default_value: Some(NumberValueNode::new(42u8).into()),
                ..StructFieldTypeNode::new("amount", NumberTypeNode::le(U8))
            }
            .into()
        )
    );
    Ok(())
}

#[test]
fn it_overrides_any_existing_default_value() -> CodamaResult<()> {
    let item: syn::Field = syn::parse_quote! {
        #[codama(default_value = 2)]
        pub amount: u8
    };
    let mut korok = FieldKorok::parse(&item)?;
    let original_field = StructFieldTypeNode {
        default_value: Some(NumberValueNode::new(1u8).into()),
        ..StructFieldTypeNode::new("amount", NumberTypeNode::le(U8))
    };
    korok.node = Some(original_field.clone().into());

    korok.accept(&mut SetDefaultValuesVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            StructFieldTypeNode {
                default_value: Some(NumberValueNode::new(2u8).into()),
                ..original_field
            }
            .into()
        )
    );
    Ok(())
}

#[test]
fn it_sets_default_values_to_instruction_argument_nodes() -> CodamaResult<()> {
    let item: syn::Field = syn::parse_quote! {
        #[codama(default_value = payer)]
        pub authority: Pubkey
    };
    let mut korok = FieldKorok::parse(&item)?;
    korok.node = Some(InstructionArgumentNode::new("authority", PublicKeyTypeNode::new()).into());

    korok.accept(&mut SetDefaultValuesVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            InstructionArgumentNode {
                default_value: Some(PayerValueNode::new().into()),
                ..InstructionArgumentNode::new("authority", PublicKeyTypeNode::new())
            }
            .into()
        )
    );
    Ok(())
}

#[test]
fn it_ignores_node_with_no_assignable_default_values() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[codama(default_value = 2)]
        pub struct Foo;
    };
    let mut korok = StructKorok::parse(&item)?;
    korok.node = Some(StructTypeNode::default().into());

    korok.accept(&mut SetDefaultValuesVisitor::new())?;
    assert_eq!(korok.node, Some(StructTypeNode::default().into()));
    Ok(())
}

#[test]
fn it_ignores_struct_field_type_nodes_with_a_contextual_default_value() -> CodamaResult<()> {
    let item: syn::Field = syn::parse_quote! {
        #[codama(default_value = payer)]
        pub authority: Pubkey
    };
    let mut korok = FieldKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut SetDefaultValuesVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(StructFieldTypeNode::new("authority", PublicKeyTypeNode::new()).into())
    );
    Ok(())
}
