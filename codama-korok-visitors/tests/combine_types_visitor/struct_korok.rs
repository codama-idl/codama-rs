use codama_errors::CodamaResult;
use codama_korok_visitors::{CombineTypesVisitor, KorokVisitable};
use codama_koroks::StructKorok;
use codama_nodes::{
    DefinedTypeNode, Node, NumberFormat::U64, NumberTypeNode, StringTypeNode, StringValueNode,
    StructFieldTypeNode, StructTypeNode, TupleTypeNode, U32, U8,
};

#[test]
fn it_creates_a_defined_type_struct_from_nammed_fields() -> CodamaResult<()> {
    let ast: syn::ItemStruct = syn::parse_quote! {
        struct Person {
            age: u8,
            name: String,
        }
    };
    let mut korok = StructKorok::parse(&ast)?;
    let struct_node = StructTypeNode::new(vec![
        StructFieldTypeNode::new("age", NumberTypeNode::le(U8)),
        StructFieldTypeNode::new("name", StringTypeNode::utf8()),
    ]);
    korok.fields.node = Some(struct_node.clone().into());

    assert_eq!(korok.node, None);
    korok.accept(&mut CombineTypesVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(DefinedTypeNode::new("person", struct_node).into())
    );
    Ok(())
}

#[test]
fn it_creates_a_defined_type_tuple_from_unnammed_fields() -> CodamaResult<()> {
    let ast: syn::ItemStruct = syn::parse_quote! {
        struct Coordinates(u32, u32);
    };
    let mut korok = StructKorok::parse(&ast)?;
    let tuple_node = TupleTypeNode::new(vec![
        NumberTypeNode::le(U32).into(),
        NumberTypeNode::le(U32).into(),
    ]);
    korok.fields.node = Some(tuple_node.clone().into());

    assert_eq!(korok.node, None);
    korok.accept(&mut CombineTypesVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(DefinedTypeNode::new("coordinates", tuple_node).into())
    );
    Ok(())
}

#[test]
fn it_creates_a_defined_type_from_single_unnammed_fields() -> CodamaResult<()> {
    let ast: syn::ItemStruct = syn::parse_quote! {
        struct Slot(u64);
    };
    let mut korok = StructKorok::parse(&ast)?;
    korok.fields.node = Some(TupleTypeNode::new(vec![NumberTypeNode::le(U64).into()]).into());

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
    let ast: syn::ItemStruct = syn::parse_quote! {
        struct MyEmptyStruct;
    };
    let mut korok = StructKorok::parse(&ast)?;

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
    let ast: syn::ItemStruct = syn::parse_quote! {
        struct Overriden(u32, u32);
    };
    let mut korok = StructKorok::parse(&ast)?;
    korok.fields.node = Some(
        TupleTypeNode::new(vec![
            NumberTypeNode::le(U32).into(),
            NumberTypeNode::le(U32).into(),
        ])
        .into(),
    );

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
fn it_fails_if_fields_do_not_resolve_to_a_type_node() -> CodamaResult<()> {
    let ast: syn::ItemStruct = syn::parse_quote! {
        struct Slot(u64);
    };
    let mut korok = StructKorok::parse(&ast)?;
    korok.fields.node = Some(StringValueNode::new("Not a type node").into());

    assert_eq!(korok.node, None);
    let error = korok.accept(&mut CombineTypesVisitor::new()).unwrap_err();
    assert_eq!(
        error.to_string(),
        "Cannot create a `definedTypeNode` from a node of kind `stringValueNode`"
    );
    Ok(())
}
