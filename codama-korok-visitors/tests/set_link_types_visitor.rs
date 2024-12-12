use codama_korok_visitors::{KorokVisitable, SetLinkTypesVisitor};
use codama_koroks::{StructKorok, TypeKorok};
use codama_nodes::{DefinedTypeLinkNode, NumberFormat::U32, NumberTypeNode};

#[test]
fn it_sets_link_nodes_using_the_type_path() {
    let ast: syn::Type = syn::parse_quote! { Membership };
    let mut korok = TypeKorok::new(&ast);

    assert_eq!(korok.node, None);
    korok.accept(&mut SetLinkTypesVisitor::new());
    assert_eq!(
        korok.node,
        Some(DefinedTypeLinkNode::new("membership").into())
    );
}

#[test]
fn it_ignores_the_path_prefix() {
    let ast: syn::Type = syn::parse_quote! { some::prefix::Membership };
    let mut korok = TypeKorok::new(&ast);

    korok.accept(&mut SetLinkTypesVisitor::new());
    assert_eq!(
        korok.node,
        Some(DefinedTypeLinkNode::new("membership").into())
    );
}

#[test]
fn it_ignores_non_path_types() {
    let ast: syn::Type = syn::parse_quote! { [u32; 8] };
    let mut korok = TypeKorok::new(&ast);

    korok.accept(&mut SetLinkTypesVisitor::new());
    assert_eq!(korok.node, None);
}

#[test]
fn it_ignores_types_that_already_have_nodes() {
    let ast: syn::Type = syn::parse_quote! { u32 };
    let mut korok = TypeKorok::new(&ast);
    korok.node = Some(NumberTypeNode::le(U32).into());

    korok.accept(&mut SetLinkTypesVisitor::new());
    korok.node = Some(NumberTypeNode::le(U32).into());
}

#[test]
fn it_works_in_any_parent_koroks() {
    let ast: syn::ItemStruct = syn::parse_quote! {
        pub struct Person {
            pub name: String,
            pub age: u8,
            pub membership: Membership,
        }
    };
    let mut korok = StructKorok::parse(&ast).unwrap();

    korok.accept(&mut SetLinkTypesVisitor::new());
    let fields = korok.fields.all;
    assert_eq!(
        fields[0].r#type.node,
        Some(DefinedTypeLinkNode::new("string").into())
    );
    assert_eq!(
        fields[1].r#type.node,
        Some(DefinedTypeLinkNode::new("u8").into())
    );
    assert_eq!(
        fields[2].r#type.node,
        Some(DefinedTypeLinkNode::new("membership").into())
    );
}
