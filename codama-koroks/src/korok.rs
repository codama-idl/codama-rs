use codama_attributes::Attributes;
use codama_nodes::Node;
use derive_more::derive::From;
use std::fmt::Debug;

pub trait KorokTrait: Debug {
    fn node(&self) -> &Option<Node>;
    fn set_node(&mut self, node: Option<Node>);
    fn attributes(&self) -> Option<&Attributes> {
        None
    }
}

#[derive(From, Debug, PartialEq)]
pub enum Korok<'a, 'b> {
    Crate(&'b crate::CrateKorok<'a>),
    Enum(&'b crate::EnumKorok<'a>),
    EnumVariant(&'b crate::EnumVariantKorok<'a>),
    Field(&'b crate::FieldKorok<'a>),
    Fields(&'b crate::FieldsKorok<'a>),
    FileModule(&'b crate::FileModuleKorok<'a>),
    Item(&'b crate::ItemKorok<'a>),
    Module(&'b crate::ModuleKorok<'a>),
    Root(&'b crate::RootKorok<'a>),
    Struct(&'b crate::StructKorok<'a>),
    Type(&'b crate::TypeKorok<'a>),
    UnsupportedItem(&'b crate::UnsupportedItemKorok<'a>),
}

#[derive(From, Debug, PartialEq)]
pub enum KorokMut<'a, 'b> {
    Crate(&'b mut crate::CrateKorok<'a>),
    Enum(&'b mut crate::EnumKorok<'a>),
    EnumVariant(&'b mut crate::EnumVariantKorok<'a>),
    Field(&'b mut crate::FieldKorok<'a>),
    Fields(&'b mut crate::FieldsKorok<'a>),
    FileModule(&'b mut crate::FileModuleKorok<'a>),
    Item(&'b mut crate::ItemKorok<'a>),
    Module(&'b mut crate::ModuleKorok<'a>),
    Root(&'b mut crate::RootKorok<'a>),
    Struct(&'b mut crate::StructKorok<'a>),
    Type(&'b mut crate::TypeKorok<'a>),
    UnsupportedItem(&'b mut crate::UnsupportedItemKorok<'a>),
}

impl KorokTrait for KorokMut<'_, '_> {
    fn node(&self) -> &Option<Node> {
        match self {
            Self::Crate(k) => k.node(),
            Self::Enum(k) => k.node(),
            Self::EnumVariant(k) => k.node(),
            Self::Field(k) => k.node(),
            Self::Fields(k) => k.node(),
            Self::FileModule(k) => k.node(),
            Self::Item(k) => k.node(),
            Self::Module(k) => k.node(),
            Self::Root(k) => k.node(),
            Self::Struct(k) => k.node(),
            Self::Type(k) => k.node(),
            Self::UnsupportedItem(k) => k.node(),
        }
    }

    fn set_node(&mut self, node: Option<Node>) {
        match self {
            Self::Crate(k) => k.set_node(node),
            Self::Enum(k) => k.set_node(node),
            Self::EnumVariant(k) => k.set_node(node),
            Self::Field(k) => k.set_node(node),
            Self::Fields(k) => k.set_node(node),
            Self::FileModule(k) => k.set_node(node),
            Self::Item(k) => k.set_node(node),
            Self::Module(k) => k.set_node(node),
            Self::Root(k) => k.set_node(node),
            Self::Struct(k) => k.set_node(node),
            Self::Type(k) => k.set_node(node),
            Self::UnsupportedItem(k) => k.set_node(node),
        }
    }

    fn attributes(&self) -> Option<&Attributes> {
        match self {
            Self::Crate(k) => k.attributes(),
            Self::Enum(k) => k.attributes(),
            Self::EnumVariant(k) => k.attributes(),
            Self::Field(k) => k.attributes(),
            Self::Fields(k) => k.attributes(),
            Self::FileModule(k) => k.attributes(),
            Self::Item(k) => k.attributes(),
            Self::Module(k) => k.attributes(),
            Self::Root(k) => k.attributes(),
            Self::Struct(k) => k.attributes(),
            Self::Type(k) => k.attributes(),
            Self::UnsupportedItem(k) => k.attributes(),
        }
    }
}
