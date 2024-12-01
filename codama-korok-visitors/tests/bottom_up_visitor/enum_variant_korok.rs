use codama_korok_visitors::{BottomUpVisitor, KorokVisitable};
use codama_koroks::EnumVariantKorok;
use codama_nodes::{
    EnumEmptyVariantTypeNode, EnumStructVariantTypeNode, EnumTupleVariantTypeNode, NumberTypeNode,
    StringTypeNode, StructFieldTypeNode, StructTypeNode, TupleTypeNode, I32, U64,
};
use codama_syn_helpers::syn_build;
use quote::quote;

#[test]
fn it_creates_enum_empty_variants() {
    let ast: syn::Variant = syn_build::parse(quote! { Foo });
    let mut korok = EnumVariantKorok::parse(&ast).unwrap();
    korok.fields.node = None;

    assert_eq!(korok.node, None);
    korok.accept(&mut BottomUpVisitor::new());
    assert_eq!(
        korok.node,
        Some(EnumEmptyVariantTypeNode::new("foo").into())
    );
}

#[test]
fn it_creates_enum_struct_variants() {
    let ast: syn::Variant = syn_build::parse(quote! { Foo { x: i32, y: i32 } });
    let mut korok = EnumVariantKorok::parse(&ast).unwrap();
    korok.fields.node = Some(
        StructTypeNode::new(vec![
            StructFieldTypeNode::new("x", NumberTypeNode::le(I32)),
            StructFieldTypeNode::new("y", NumberTypeNode::le(I32)),
        ])
        .into(),
    );

    assert_eq!(korok.node, None);
    korok.accept(&mut BottomUpVisitor::new());
    assert_eq!(
        korok.node,
        Some(
            EnumStructVariantTypeNode::new(
                "foo",
                StructTypeNode::new(vec![
                    StructFieldTypeNode::new("x", NumberTypeNode::le(I32)),
                    StructFieldTypeNode::new("y", NumberTypeNode::le(I32)),
                ])
            )
            .into()
        )
    );
}

#[test]
fn it_creates_enum_tuple_variants() {
    let ast: syn::Variant = syn_build::parse(quote! { Foo (u64, String) });
    let mut korok = EnumVariantKorok::parse(&ast).unwrap();
    korok.fields.node = Some(
        TupleTypeNode::new(vec![
            NumberTypeNode::le(U64).into(),
            StringTypeNode::utf8().into(),
        ])
        .into(),
    );

    assert_eq!(korok.node, None);
    korok.accept(&mut BottomUpVisitor::new());
    assert_eq!(
        korok.node,
        Some(
            EnumTupleVariantTypeNode::new(
                "foo",
                TupleTypeNode::new(vec![
                    NumberTypeNode::le(U64).into(),
                    StringTypeNode::utf8().into(),
                ])
            )
            .into()
        )
    );
}

#[test]
fn it_keeps_track_of_the_variant_discriminant() {
    let ast: syn::Variant = syn_build::parse(quote! { Foo = 42 });
    let mut korok = EnumVariantKorok::parse(&ast).unwrap();
    korok.fields.node = None;

    assert_eq!(korok.node, None);
    korok.accept(&mut BottomUpVisitor::new());
    assert_eq!(
        korok.node,
        Some(
            EnumEmptyVariantTypeNode {
                name: "foo".into(),
                discriminator: Some(42)
            }
            .into()
        )
    );
}

#[test]
fn it_does_not_override_existing_nodes_by_default() {
    let ast: syn::Variant = syn_build::parse(quote! { Foo });
    let mut korok = EnumVariantKorok::parse(&ast).unwrap();
    korok.fields.node = None;

    korok.node = Some(EnumEmptyVariantTypeNode::new("bar").into());
    korok.accept(&mut BottomUpVisitor::new());
    assert_eq!(
        korok.node,
        Some(EnumEmptyVariantTypeNode::new("bar").into())
    );
}
