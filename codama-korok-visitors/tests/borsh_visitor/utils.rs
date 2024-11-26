use codama_korok_visitors::{BorshVisitor, KorokVisitable};
use codama_koroks::{stores::RootStore, RootKorok};
use codama_nodes::Node;
use proc_macro2::TokenStream;
use quote::quote;

pub fn get_node_from_struct(tt: TokenStream) -> Option<Node> {
    let store = RootStore::populate_from(tt).unwrap();
    let mut korok = RootKorok::parse(&store).unwrap();
    korok.accept(&mut BorshVisitor::new());
    korok.first_item().as_struct().unwrap().node.clone()
}

pub fn get_node_from_enum(tt: TokenStream) -> Option<Node> {
    let store = RootStore::populate_from(tt).unwrap();
    let mut korok = RootKorok::parse(&store).unwrap();
    korok.accept(&mut BorshVisitor::new());
    korok.first_item().as_enum().unwrap().node.clone()
}

pub fn get_node_from_enum_variant(tt: TokenStream) -> Option<Node> {
    let store = RootStore::populate_from(quote! { pub enum Foo { #tt } }).unwrap();
    let mut korok = RootKorok::parse(&store).unwrap();
    korok.accept(&mut BorshVisitor::new());
    korok.first_item().as_enum().unwrap().variants[0]
        .node
        .clone()
}

pub fn get_node_from_type(tt: TokenStream) -> Option<Node> {
    let store = RootStore::populate_from(quote! { pub struct Foo(#tt); }).unwrap();
    let mut korok = RootKorok::parse(&store).unwrap();
    korok.accept(&mut BorshVisitor::new());
    korok.first_item().as_struct().unwrap().fields[0]
        .node
        .clone()
}
