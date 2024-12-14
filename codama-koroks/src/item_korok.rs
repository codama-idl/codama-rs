use crate::{
    EnumKorok, FileModuleKorok, KorokTrait, ModuleKorok, StructKorok, UnsupportedItemKorok,
};
use codama_attributes::Attributes;
use codama_errors::{CodamaResult, IteratorCombineErrors};
use codama_nodes::Node;
use codama_stores::FileModuleStore;
use std::ops::AddAssign;

#[derive(Debug, PartialEq)]
pub enum ItemKorok<'a> {
    FileModule(FileModuleKorok<'a>),
    Module(ModuleKorok<'a>),
    Struct(StructKorok<'a>),
    Enum(EnumKorok<'a>),
    Unsupported(UnsupportedItemKorok<'a>),
}

impl<'a> ItemKorok<'a> {
    pub fn parse(
        item: &'a syn::Item,
        file_modules: &'a [FileModuleStore],
        file_module_index: &mut usize,
    ) -> CodamaResult<Self> {
        match item {
            syn::Item::Mod(ast) if ast.content.is_none() => {
                match file_modules.get(*file_module_index) {
                    Some(module) => {
                        file_module_index.add_assign(1);
                        Ok(ItemKorok::FileModule(FileModuleKorok::parse(ast, module)?))
                    }
                    None => {
                        Err(syn::Error::new_spanned(ast, "Associated ModuleStore not found").into())
                    }
                }
            }
            syn::Item::Mod(ast) if ast.content.is_some() => Ok(ItemKorok::Module(
                ModuleKorok::parse(ast, file_modules, file_module_index)?,
            )),
            syn::Item::Struct(ast) => Ok(ItemKorok::Struct(StructKorok::parse(ast)?)),
            syn::Item::Enum(ast) => Ok(ItemKorok::Enum(EnumKorok::parse(ast)?)),
            _ => Ok(ItemKorok::Unsupported(UnsupportedItemKorok::parse(item)?)),
        }
    }

    pub fn parse_all(
        items: &'a [syn::Item],
        file_modules: &'a [FileModuleStore],
        file_module_index: &mut usize,
    ) -> CodamaResult<Vec<Self>> {
        items
            .iter()
            .map(|item| Self::parse(item, file_modules, file_module_index))
            .collect_and_combine_errors()
    }
}

impl KorokTrait for ItemKorok<'_> {
    fn node(&self) -> &Option<Node> {
        match self {
            ItemKorok::Struct(k) => k.node(),
            ItemKorok::Enum(k) => k.node(),
            ItemKorok::FileModule(k) => k.node(),
            ItemKorok::Module(k) => k.node(),
            ItemKorok::Unsupported(k) => k.node(),
        }
    }

    fn set_node(&mut self, node: Option<Node>) {
        match self {
            ItemKorok::Struct(k) => k.set_node(node),
            ItemKorok::Enum(k) => k.set_node(node),
            ItemKorok::FileModule(k) => k.set_node(node),
            ItemKorok::Module(k) => k.set_node(node),
            ItemKorok::Unsupported(k) => k.set_node(node),
        }
    }

    fn attributes(&self) -> Option<&Attributes> {
        match self {
            ItemKorok::Struct(k) => k.attributes(),
            ItemKorok::Enum(k) => k.attributes(),
            ItemKorok::FileModule(k) => k.attributes(),
            ItemKorok::Module(k) => k.attributes(),
            ItemKorok::Unsupported(k) => k.attributes(),
        }
    }
}
