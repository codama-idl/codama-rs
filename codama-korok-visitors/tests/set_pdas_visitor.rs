use codama_errors::CodamaResult;
use codama_korok_visitors::{
    ApplyTypeOverridesVisitor, IdentifyFieldTypesVisitor, KorokVisitable, SetPdasVisitor,
};
use codama_koroks::{EnumKorok, StructKorok};
use codama_nodes::{
    ConstantPdaSeedNode, NumberFormat::U8, NumberTypeNode, PdaNode, PublicKeyTypeNode,
    StringTypeNode, StringValueNode, VariablePdaSeedNode,
};

#[test]
fn it_defines_pdas_from_structs() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaPda)]
        #[codama(seed(type = string(utf8), value = "counter_pda"))]
        #[codama(seed(name = "authority", type = public_key))]
        #[codama(seed(name = "identifier", type = number(u8)))]
        struct Counter;
    };
    let mut korok = StructKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut SetPdasVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            PdaNode::new(
                "counter",
                vec![
                    ConstantPdaSeedNode::new(
                        StringTypeNode::utf8(),
                        StringValueNode::new("counter_pda")
                    )
                    .into(),
                    VariablePdaSeedNode::new("authority", PublicKeyTypeNode::new()).into(),
                    VariablePdaSeedNode::new("identifier", NumberTypeNode::le(U8)).into(),
                ]
            )
            .into()
        )
    );
    Ok(())
}

#[test]
fn it_defines_pdas_from_structs_with_linked_fields() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaPda)]
        #[codama(seed(type = string(utf8), value = "counter_pda"))]
        #[codama(seed(name = "authority"))]
        #[codama(seed(name = "identifier"))]
        struct Counter {
            identifier: u8,
            authority: Pubkey,
        }
    };
    let mut korok = StructKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut SetPdasVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            PdaNode::new(
                "counter",
                vec![
                    ConstantPdaSeedNode::new(
                        StringTypeNode::utf8(),
                        StringValueNode::new("counter_pda")
                    )
                    .into(),
                    VariablePdaSeedNode::new("authority", PublicKeyTypeNode::new()).into(),
                    VariablePdaSeedNode::new("identifier", NumberTypeNode::le(U8)).into(),
                ]
            )
            .into()
        )
    );
    Ok(())
}

#[test]
fn it_defines_pdas_from_structs_with_linked_fields_and_custom_nodes() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaPda)]
        #[codama(seed(type = string(utf8), value = "counter_pda"))]
        #[codama(seed(name = "authority"))]
        #[codama(seed(name = "identifier"))]
        struct Counter {
            identifier: Overridden,
            authority: Overridden,
        }
    };
    let mut korok = StructKorok::parse(&item)?;
    korok.fields[0].node = Some(NumberTypeNode::le(U8).into());
    korok.fields[1].node = Some(PublicKeyTypeNode::new().into());

    assert_eq!(korok.node, None);
    korok.accept(&mut SetPdasVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            PdaNode::new(
                "counter",
                vec![
                    ConstantPdaSeedNode::new(
                        StringTypeNode::utf8(),
                        StringValueNode::new("counter_pda")
                    )
                    .into(),
                    VariablePdaSeedNode::new("authority", PublicKeyTypeNode::new()).into(),
                    VariablePdaSeedNode::new("identifier", NumberTypeNode::le(U8)).into(),
                ]
            )
            .into()
        )
    );
    Ok(())
}

#[test]
fn it_defines_pdas_from_structs_with_linked_fields_and_type_overrides() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaPda)]
        #[codama(seed(type = string(utf8), value = "counter_pda"))]
        #[codama(seed(name = "authority"))]
        #[codama(seed(name = "identifier"))]
        struct Counter {
            #[codama(type = number(u8))]
            identifier: Overridden,
            #[codama(type = public_key)]
            authority: Overridden,
        }
    };
    let mut korok = StructKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut ApplyTypeOverridesVisitor::new())?;
    korok.accept(&mut SetPdasVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            PdaNode::new(
                "counter",
                vec![
                    ConstantPdaSeedNode::new(
                        StringTypeNode::utf8(),
                        StringValueNode::new("counter_pda")
                    )
                    .into(),
                    VariablePdaSeedNode::new("authority", PublicKeyTypeNode::new()).into(),
                    VariablePdaSeedNode::new("identifier", NumberTypeNode::le(U8)).into(),
                ]
            )
            .into()
        )
    );
    Ok(())
}

#[test]
fn it_defines_pdas_from_enums() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaPda)]
        #[codama(seed(type = string(utf8), value = "counter_pda"))]
        #[codama(seed(name = "authority", type = public_key))]
        #[codama(seed(name = "identifier", type = number(u8)))]
        enum Counter { One, Two, Three }
    };
    let mut korok = EnumKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut SetPdasVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            PdaNode::new(
                "counter",
                vec![
                    ConstantPdaSeedNode::new(
                        StringTypeNode::utf8(),
                        StringValueNode::new("counter_pda")
                    )
                    .into(),
                    VariablePdaSeedNode::new("authority", PublicKeyTypeNode::new()).into(),
                    VariablePdaSeedNode::new("identifier", NumberTypeNode::le(U8)).into(),
                ]
            )
            .into()
        )
    );
    Ok(())
}
