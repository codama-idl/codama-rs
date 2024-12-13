use codama_nodes::Node;
use derive_more::derive::From;
use std::fmt::Debug;

pub trait KorokTrait: Debug {
    fn node(&self) -> &Option<Node>;
    fn set_node(&mut self, node: Option<Node>);
}

#[derive(From, Debug)]
pub enum Korok<'a> {
    Crate(crate::CrateKorok<'a>),
    Enum(crate::EnumKorok<'a>),
    EnumVariant(crate::EnumVariantKorok<'a>),
    Field(crate::FieldKorok<'a>),
    Fields(crate::FieldsKorok<'a>),
    FileModule(crate::FileModuleKorok<'a>),
    Item(crate::ItemKorok<'a>),
    Module(crate::ModuleKorok<'a>),
    Root(crate::RootKorok<'a>),
    Struct(crate::StructKorok<'a>),
    Type(crate::TypeKorok<'a>),
    UnsupportedItem(crate::UnsupportedItemKorok<'a>),
}

impl KorokTrait for Korok<'_> {
    fn node(&self) -> &Option<Node> {
        match self {
            Korok::Crate(k) => k.node(),
            Korok::Enum(k) => k.node(),
            Korok::EnumVariant(k) => k.node(),
            Korok::Field(k) => k.node(),
            Korok::Fields(k) => k.node(),
            Korok::FileModule(k) => k.node(),
            Korok::Item(k) => k.node(),
            Korok::Module(k) => k.node(),
            Korok::Root(k) => k.node(),
            Korok::Struct(k) => k.node(),
            Korok::Type(k) => k.node(),
            Korok::UnsupportedItem(k) => k.node(),
        }
    }

    fn set_node(&mut self, node: Option<Node>) {
        match self {
            Korok::Crate(k) => k.set_node(node),
            Korok::Enum(k) => k.set_node(node),
            Korok::EnumVariant(k) => k.set_node(node),
            Korok::Field(k) => k.set_node(node),
            Korok::Fields(k) => k.set_node(node),
            Korok::FileModule(k) => k.set_node(node),
            Korok::Item(k) => k.set_node(node),
            Korok::Module(k) => k.set_node(node),
            Korok::Root(k) => k.set_node(node),
            Korok::Struct(k) => k.set_node(node),
            Korok::Type(k) => k.set_node(node),
            Korok::UnsupportedItem(k) => k.set_node(node),
        }
    }
}
