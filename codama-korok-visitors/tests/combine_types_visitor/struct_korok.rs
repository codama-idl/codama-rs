use codama_errors::CodamaResult;
use codama_korok_visitors::{CombineTypesVisitor, KorokVisitable};
use codama_koroks::StructKorok;
use codama_nodes::{
    DefinedTypeNode, Node, NumberFormat::U64, NumberTypeNode, StringTypeNode, StringValueNode,
    StructFieldTypeNode, StructTypeNode, TupleTypeNode, U32, U8,
};

#[test]
fn it_creates_a_defined_type_struct_from_nammed_fields() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        struct Person {
            age: u8,
            name: String,
        }
    };
    let mut korok = StructKorok::parse(&item)?;
    korok.fields[0].node = Some(StructFieldTypeNode::new("age", NumberTypeNode::le(U8)).into());
    korok.fields[1].node = Some(StructFieldTypeNode::new("name", StringTypeNode::utf8()).into());

    assert_eq!(korok.node, None);
    korok.accept(&mut CombineTypesVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            DefinedTypeNode::new(
                "person",
                StructTypeNode::new(vec![
                    StructFieldTypeNode::new("age", NumberTypeNode::le(U8)),
                    StructFieldTypeNode::new("name", StringTypeNode::utf8()),
                ])
            )
            .into()
        )
    );
    Ok(())
}

#[test]
fn it_creates_a_defined_type_tuple_from_unnammed_fields() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        struct Coordinates(u32, u32);
    };
    let mut korok = StructKorok::parse(&item)?;
    korok.fields[0].node = Some(NumberTypeNode::le(U32).into());
    korok.fields[1].node = Some(NumberTypeNode::le(U32).into());

    assert_eq!(korok.node, None);
    korok.accept(&mut CombineTypesVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            DefinedTypeNode::new(
                "coordinates",
                TupleTypeNode::new(vec![
                    NumberTypeNode::le(U32).into(),
                    NumberTypeNode::le(U32).into(),
                ])
            )
            .into()
        )
    );
    Ok(())
}

#[test]
fn it_creates_a_defined_type_from_single_unnammed_fields() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        struct Slot(u64);
    };
    let mut korok = StructKorok::parse(&item)?;
    korok.fields[0].node = Some(NumberTypeNode::le(U64).into());

    assert_eq!(korok.node, None);
    korok.accept(&mut CombineTypesVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(DefinedTypeNode::new("slot", NumberTypeNode::le(U64)).into())
    );
    Ok(())
}

#[test]
fn it_returns_an_empty_struct_from_unit_fields() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        struct MyEmptyStruct;
    };
    let mut korok = StructKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut CombineTypesVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(DefinedTypeNode::new("myEmptyStruct", StructTypeNode::new(vec![])).into())
    );
    Ok(())
}

#[test]
fn it_does_not_override_existing_nodes_by_default() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        struct Overriden(u32, u32);
    };
    let mut korok = StructKorok::parse(&item)?;
    korok.fields[0].node = Some(NumberTypeNode::le(U32).into());
    korok.fields[1].node = Some(NumberTypeNode::le(U32).into());

    let original_node = Some(Node::from(DefinedTypeNode::new(
        "original",
        StringTypeNode::utf8(),
    )));
    korok.node = original_node.clone();
    korok.accept(&mut CombineTypesVisitor::new())?;
    assert_eq!(korok.node, original_node);
    Ok(())
}

#[test]
fn it_fails_if_fields_are_missing() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        struct Person {
            age: u64,
        }
    };
    let mut korok = StructKorok::parse(&item)?;
    korok.fields[0].node = None;

    assert_eq!(korok.node, None);
    let error = korok
        .accept(&mut CombineTypesVisitor::strict())
        .unwrap_err();
    assert_eq!(
        error.to_string(),
        "Field `age` in struct `Person` does not resolve to a `structFieldTypeNode`"
    );
    Ok(())
}

#[test]
fn it_fails_if_fields_do_not_resolve_to_type_nodes() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        struct Slot(u64);
    };
    let mut korok = StructKorok::parse(&item)?;
    korok.fields[0].node = Some(StringValueNode::new("Not a type node").into());

    assert_eq!(korok.node, None);
    let error = korok
        .accept(&mut CombineTypesVisitor::strict())
        .unwrap_err();
    assert_eq!(
        error.to_string(),
        "Field `0` in struct `Slot` does not resolve to a `TypeNode`"
    );
    Ok(())
}
