use crate::ItemKorok;
use codama_errors::CodamaResult;
use codama_nodes::Node;
use codama_stores::FileModuleStore;
use std::path::Path;

#[derive(Debug)]
pub struct FileModuleKorok<'a> {
    pub ast: &'a syn::ItemMod,
    pub file: &'a syn::File,
    pub items: Vec<ItemKorok<'a>>,
    pub node: Option<Node>,
    pub path: &'a Path,
}

impl<'a> FileModuleKorok<'a> {
    pub fn parse(ast: &'a syn::ItemMod, module: &'a FileModuleStore) -> CodamaResult<Self> {
        if let Some(_) = ast.content {
            return Err(syn::Error::new_spanned(
                ast,
                "Module has content, it should be parsed with ModuleKorok",
            )
            .into());
        }

        Ok(Self {
            ast,
            file: &module.file,
            items: ItemKorok::parse_all(&module.file.items, &module.file_modules)?,
            path: &module.path,
            node: None,
        })
    }
}
