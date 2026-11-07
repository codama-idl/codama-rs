use codama_errors::CodamaResult;
use codama_korok_visitors::{IdentifyFieldTypesVisitor, KorokVisitable, SetEventsVisitor};
use codama_koroks::{EnumKorok, StructKorok};
use codama_nodes::{
    BooleanTypeNode, EventNode, FieldDiscriminatorNode, IntegerFormat::U64, IntegerTypeNode,
    PublicKeyTypeNode, SizeDiscriminatorNode, StructFieldTypeNode, StructTypeNode,
};

#[test]
fn from_struct() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaEvent)]
        struct TransferEvent {
            authority: Pubkey,
            amount: u64,
        }
    };
    let mut korok = StructKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut SetEventsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            EventNode::new(
                "transfer_event",
                StructTypeNode::new(vec![
                    StructFieldTypeNode::new("authority", PublicKeyTypeNode::new()),
                    StructFieldTypeNode::new("amount", IntegerTypeNode::le(U64)),
                ])
            )
            .into()
        )
    );
    Ok(())
}

#[test]
fn from_empty_struct() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaEvent)]
        struct PingEvent;
    };
    let mut korok = StructKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut SetEventsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(EventNode::new("ping_event", StructTypeNode::new(vec![])).into())
    );
    Ok(())
}

#[test]
fn from_enum() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaEvent)]
        enum SomeEnum {
            A,
            B,
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut SetEventsVisitor::new())?;
    assert_eq!(korok.node, None);
    // No visitor error because there is already a compilation error.
    Ok(())
}

#[test]
fn no_overrides() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaEvent)]
        struct MyEvent;
    };
    let mut korok = StructKorok::parse(&item)?;
    korok.node = Some(BooleanTypeNode::default().into());

    korok.accept(&mut SetEventsVisitor::new())?;
    assert_eq!(korok.node, Some(BooleanTypeNode::default().into()));
    Ok(())
}

#[test]
fn with_name_directive() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaEvent)]
        #[codama(identifier = "transfer")]
        struct TransferEvent;
    };
    let mut korok = StructKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut SetEventsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(EventNode::new("transfer", StructTypeNode::new(vec![])).into())
    );
    Ok(())
}

#[test]
fn with_discriminator_directives() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaEvent)]
        #[codama(discriminator(size = 100))]
        #[codama(discriminator(field = "discriminator"))]
        struct MyEvent;
    };
    let mut korok = StructKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut SetEventsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            EventNode {
                discriminators: vec![
                    SizeDiscriminatorNode::new(100).into(),
                    FieldDiscriminatorNode::new("discriminator", 0).into(),
                ],
                ..EventNode::new("my_event", StructTypeNode::new(vec![]))
            }
            .into()
        )
    );
    Ok(())
}
