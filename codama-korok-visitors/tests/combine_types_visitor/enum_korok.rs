use codama_korok_visitors::{CombineTypesVisitor, KorokVisitable};
use codama_koroks::EnumKorok;
use codama_nodes::{
    DefinedTypeNode, EnumEmptyVariantTypeNode, EnumStructVariantTypeNode, EnumTupleVariantTypeNode,
    EnumTypeNode, Node, NumberTypeNode, StringTypeNode, StructFieldTypeNode, StructTypeNode,
    TupleTypeNode, I32,
};

#[test]
fn it_creates_a_defined_type_enum_from_variants() -> syn::Result<()> {
    let ast: syn::ItemEnum = syn::parse_quote! {
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
        }
    };
    let mut korok = EnumKorok::parse(&ast).unwrap();
    let variant_quit = EnumEmptyVariantTypeNode::new("quit");
    let variant_move = EnumStructVariantTypeNode::new(
        "move",
        StructTypeNode::new(vec![
            StructFieldTypeNode::new("x", NumberTypeNode::le(I32)),
            StructFieldTypeNode::new("y", NumberTypeNode::le(I32)),
        ]),
    );
    let variant_write = EnumTupleVariantTypeNode::new(
        "write",
        TupleTypeNode::new(vec![StringTypeNode::utf8().into()]),
    );
    korok.variants[0].node = Some(variant_quit.clone().into());
    korok.variants[1].node = Some(variant_move.clone().into());
    korok.variants[2].node = Some(variant_write.clone().into());

    assert_eq!(korok.node, None);
    korok.accept(&mut CombineTypesVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            DefinedTypeNode::new(
                "message",
                EnumTypeNode::new(vec![
                    variant_quit.into(),
                    variant_move.into(),
                    variant_write.into(),
                ])
            )
            .into()
        )
    );
    Ok(())
}

#[test]
fn it_does_not_override_existing_nodes_by_default() -> syn::Result<()> {
    let ast: syn::ItemEnum = syn::parse_quote! { enum Direction { Left } };
    let mut korok = EnumKorok::parse(&ast).unwrap();
    korok.variants[0].node = Some(EnumEmptyVariantTypeNode::new("left").into());

    let original_node = Some(Node::from(DefinedTypeNode::new(
        "direction",
        EnumTypeNode::new(vec![EnumEmptyVariantTypeNode::new("right").into()]),
    )));
    korok.node = original_node.clone();
    korok.accept(&mut CombineTypesVisitor::new())?;
    assert_eq!(korok.node, original_node);
    Ok(())
}
