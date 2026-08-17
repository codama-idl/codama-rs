use codama_errors::CodamaResult;
use codama_korok_visitors::{
    ApplyDisplayVisitor, IdentifyFieldTypesVisitor, KorokVisitable, SetEventsVisitor,
};
use codama_koroks::EnumKorok;
use codama_nodes::{
    AmountNumberDisplayNode, DisplaySkip, NumberDisplayNode, NumberFormat::U64, NumberTypeNode,
    NumberValueNode, StringValueNode, StructFieldDisplayNode, StructFieldTypeNode,
};

fn event_fields(item: syn::Item) -> CodamaResult<Vec<StructFieldTypeNode>> {
    let mut korok = EnumKorok::parse(&item)?;

    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut ApplyDisplayVisitor::new())?;
    korok.accept(&mut SetEventsVisitor::new())?;

    let Some(codama_nodes::Node::Program(program)) = korok.node else {
        panic!("Expected a program node");
    };
    let codama_nodes::TypeNode::Struct(data) = &*program.events[0].data else {
        panic!("Expected a struct type node");
    };
    Ok(data.fields.clone())
}

#[test]
fn it_preserves_tuple_field_displays() -> CodamaResult<()> {
    let fields = event_fields(syn::parse_quote! {
        #[derive(CodamaEvents)]
        enum Events {
            Withdraw(
                #[codama(name = "amount")]
                #[codama(display(label = "First", skip = always))]
                #[codama(display(label = "Amount", flatten, flatten_prefix = "Withdraw "))]
                u64,
            ),
        }
    })?;

    assert_eq!(
        fields[1].display,
        Some(StructFieldDisplayNode {
            label: Some("Amount".to_string()),
            skip: Some(DisplaySkip::Always),
            flatten: Some(true),
            flatten_prefix: Some("Withdraw ".to_string()),
        })
    );
    Ok(())
}

#[test]
fn it_leaves_tuple_fields_without_displays_untouched() -> CodamaResult<()> {
    let fields = event_fields(syn::parse_quote! {
        #[derive(CodamaEvents)]
        enum Events {
            Withdraw(#[codama(name = "amount")] u64),
        }
    })?;

    assert_eq!(fields[1].display, None);
    Ok(())
}

#[test]
fn it_preserves_both_halves_of_a_tuple_field_display() -> CodamaResult<()> {
    let fields = event_fields(syn::parse_quote! {
        #[derive(CodamaEvents)]
        enum Events {
            Withdraw(
                #[codama(name = "amount")]
                #[codama(display(label = "Amount", amount(decimals = 9, unit = "SOL")))]
                u64,
            ),
        }
    })?;

    assert_eq!(
        fields[1].display,
        Some(StructFieldDisplayNode::new("Amount"))
    );
    assert_eq!(
        *fields[1].r#type,
        NumberTypeNode {
            display: Box::new(Some(NumberDisplayNode::Amount(
                AmountNumberDisplayNode::new(
                    NumberValueNode::new(9u64),
                    StringValueNode::new("SOL"),
                ),
            ))),
            ..NumberTypeNode::le(U64)
        }
        .into()
    );
    Ok(())
}
