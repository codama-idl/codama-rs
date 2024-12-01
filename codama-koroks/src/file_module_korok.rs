use crate::ItemKorok;
use codama_errors::CodamaResult;
use codama_nodes::Node;
use codama_stores::FileModuleStore;

#[derive(Debug)]
pub struct FileModuleKorok<'a> {
    pub ast: &'a syn::ItemMod,
    pub items: Vec<ItemKorok<'a>>,
    pub node: Option<Node>,
    pub store: &'a FileModuleStore,
}

impl<'a> FileModuleKorok<'a> {
    pub fn parse(ast: &'a syn::ItemMod, module_store: &'a FileModuleStore) -> CodamaResult<Self> {
        if let Some(_) = ast.content {
            return Err(syn::Error::new_spanned(
                ast,
                "Module has content, it should be parsed with ModuleKorok",
            )
            .into());
        }

        Ok(Self {
            ast,
            items: ItemKorok::parse_all(&module_store.file.items, &module_store.file_modules)?,
            node: None,
            store: module_store,
        })
    }
}
