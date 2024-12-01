use crate::{EnumKorok, FileModuleKorok, ModuleKorok, StructKorok, UnsupportedItemKorok};
use codama_errors::CodamaResult;
use codama_nodes::Node;
use codama_stores::FileModuleStore;

#[derive(Debug)]
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
        modules: &'a Vec<FileModuleStore>,
        item_index: usize,
    ) -> CodamaResult<Self> {
        match item {
            syn::Item::Mod(ast) if ast.content.is_none() => {
                let module = modules.iter().nth(item_index);
                match module {
                    Some(module) => Ok(ItemKorok::FileModule(FileModuleKorok::parse(ast, module)?)),
                    None => {
                        Err(syn::Error::new_spanned(ast, "Associated ModuleStore not found").into())
                    }
                }
            }
            syn::Item::Mod(ast) if ast.content.is_some() => {
                Ok(ItemKorok::Module(ModuleKorok::parse(ast, modules)?))
            }
            syn::Item::Struct(ast) => Ok(ItemKorok::Struct(StructKorok::parse(ast)?)),
            syn::Item::Enum(ast) => Ok(ItemKorok::Enum(EnumKorok::parse(ast)?)),
            _ => Ok(ItemKorok::Unsupported(UnsupportedItemKorok {
                ast: item,
                node: None,
            })),
        }
    }

    pub fn parse_all(
        items: &'a Vec<syn::Item>,
        modules: &'a Vec<FileModuleStore>,
    ) -> CodamaResult<Vec<Self>> {
        items
            .iter()
            .enumerate()
            .map(|(item_index, item)| Self::parse(item, modules, item_index))
            .collect()
    }

    pub fn node(&self) -> Option<Node> {
        match self {
            ItemKorok::Struct(k) => k.node.clone(),
            ItemKorok::Enum(k) => k.node.clone(),
            ItemKorok::FileModule(k) => k.node.clone(),
            ItemKorok::Module(k) => k.node.clone(),
            ItemKorok::Unsupported(k) => k.node.clone(),
        }
    }

    pub fn set_node(&mut self, node: Option<Node>) {
        match self {
            ItemKorok::Struct(k) => k.node = node,
            ItemKorok::Enum(k) => k.node = node,
            ItemKorok::FileModule(k) => k.node = node,
            ItemKorok::Module(k) => k.node = node,
            ItemKorok::Unsupported(k) => k.node = node,
        }
    }
}
