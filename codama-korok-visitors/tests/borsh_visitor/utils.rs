use codama_korok_visitors::{BorshVisitor, BottomUpVisitor, KorokVisitable};
use codama_koroks::{ItemKorok, RootKorok};
use codama_nodes::Node;
use codama_stores::RootStore;
use proc_macro2::TokenStream;
use quote::quote;

pub fn get_node(tt: TokenStream, node_getter: fn(RootKorok) -> Option<Node>) -> Option<Node> {
    let store = RootStore::populate_from(tt).unwrap();
    let mut korok = RootKorok::parse(&store).unwrap();
    korok.accept(&mut BorshVisitor::new());
    korok.accept(&mut BottomUpVisitor::new());
    node_getter(korok)
}

pub fn get_node_from_item(tt: TokenStream) -> Option<Node> {
    get_node(tt, |k| (&k.crates[0].items[0]).node())
}

pub fn get_node_from_enum_variant(tt: TokenStream) -> Option<Node> {
    get_node(quote! { pub enum Foo { #tt } }, |k| {
        match &k.crates[0].items[0] {
            ItemKorok::Enum(k) => k.variants[0].node.clone(),
            _ => None,
        }
    })
}

pub fn get_node_from_type(tt: TokenStream) -> Option<Node> {
    get_node(quote! { pub struct Foo(#tt); }, |k| {
        match &k.crates[0].items[0] {
            ItemKorok::Struct(k) => k.fields.all[0].node.clone(),
            _ => None,
        }
    })
}
