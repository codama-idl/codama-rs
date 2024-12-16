use codama_korok_visitors::{FilterItemsVisitor, KorokVisitable, KorokVisitor, UniformVisitor};
use codama_koroks::{ItemKorok, KorokTrait};
use codama_nodes::PublicKeyTypeNode;

#[test]
fn it_only_starts_the_child_visitor_on_filtered_items() {
    let ast: syn::Item = syn::parse_quote! {
        mod parent {
            mod foo {
                pub struct Foo(u32);
            }
            mod bar {
                pub struct Bar(u32);
            }
        }
    };
    let mut korok = ItemKorok::parse(&ast, &[], &mut 0).unwrap();

    korok.accept(&mut FilterItemsVisitor::new(
        |item| match item {
            ItemKorok::Module(module) => module.ast.ident == "foo",
            _ => false,
        },
        UniformVisitor::new(|mut k, visitor| {
            visitor.visit_children(&mut k);
            k.set_node(Some(PublicKeyTypeNode::new().into()))
        }),
    ));

    let ItemKorok::Module(module) = &korok else {
        panic!("Expected parent module");
    };
    let ItemKorok::Module(foo) = &module.items[0] else {
        panic!("Expected foo module");
    };
    let ItemKorok::Struct(foo_struct) = &foo.items[0] else {
        panic!("Expected foo struct");
    };
    let ItemKorok::Module(bar) = &module.items[1] else {
        panic!("Expected bar module");
    };
    let ItemKorok::Struct(bar_struct) = &bar.items[0] else {
        panic!("Expected bar struct");
    };

    assert_eq!(foo.node, Some(PublicKeyTypeNode::new().into()));
    assert_eq!(foo_struct.node, Some(PublicKeyTypeNode::new().into()));
    assert_eq!(
        foo_struct.fields.node,
        Some(PublicKeyTypeNode::new().into())
    );
    assert_eq!(
        foo_struct.fields.all[0].node,
        Some(PublicKeyTypeNode::new().into())
    );
    assert_eq!(
        foo_struct.fields.all[0].r#type.node,
        Some(PublicKeyTypeNode::new().into())
    );

    assert_eq!(bar.node, None);
    assert_eq!(bar_struct.node, None);
    assert_eq!(bar_struct.fields.node, None);
    assert_eq!(bar_struct.fields.all[0].node, None);
    assert_eq!(bar_struct.fields.all[0].r#type.node, None);
}
