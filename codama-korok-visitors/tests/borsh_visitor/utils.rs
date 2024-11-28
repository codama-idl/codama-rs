use codama_korok_visitors::{BorshVisitor, KorokVisitable};
use codama_koroks::{stores::RootStore, ItemKorok, RootKorok};
use codama_nodes::Node;
use proc_macro2::TokenStream;
use quote::quote;

pub fn get_node(tt: TokenStream, node_getter: fn(RootKorok) -> Option<Node>) -> Option<Node> {
    let store = RootStore::populate_from(tt).unwrap();
    let mut korok = RootKorok::parse(&store).unwrap();
    korok.accept(&mut BorshVisitor::new());
    node_getter(korok)
}

pub fn get_node_from_item(tt: TokenStream) -> Option<Node> {
    get_node(tt, |k| k.first_item().node())
}

pub fn get_node_from_enum_variant(tt: TokenStream) -> Option<Node> {
    get_node(quote! { pub enum Foo { #tt } }, |k| match k.first_item() {
        ItemKorok::Enum(k) => k.variants[0].node.clone(),
        _ => None,
    })
}

pub fn get_node_from_type(tt: TokenStream) -> Option<Node> {
    get_node(quote! { pub struct Foo(#tt); }, |k| match k.first_item() {
        ItemKorok::Struct(k) => k.fields.all[0].node.clone(),
        _ => None,
    })
}
