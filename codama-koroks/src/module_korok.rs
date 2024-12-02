use crate::ItemKorok;
use codama_errors::CodamaResult;
use codama_nodes::Node;
use codama_stores::FileModuleStore;

#[derive(Debug)]
pub struct ModuleKorok<'a> {
    pub ast: &'a syn::ItemMod,
    pub items: Vec<ItemKorok<'a>>,
    pub node: Option<Node>,
}

impl<'a> ModuleKorok<'a> {
    pub fn parse(
        ast: &'a syn::ItemMod,
        file_modules: &'a Vec<FileModuleStore>,
        file_module_index: &mut usize,
    ) -> CodamaResult<Self> {
        match &ast.content {
            Some(content) => Ok(Self {
                ast,
                items: ItemKorok::parse_all(&content.1, file_modules, file_module_index)?,
                node: None,
            }),
            None => Err(syn::Error::new_spanned(
                ast,
                "Module has no content, it should be parsed with FileModuleKorok",
            )
            .into()),
        }
    }
}
