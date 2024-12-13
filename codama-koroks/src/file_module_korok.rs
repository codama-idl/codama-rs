use crate::{ItemKorok, Korok};
use codama_attributes::Attributes;
use codama_errors::CodamaResult;
use codama_nodes::Node;
use codama_stores::FileModuleStore;

#[derive(Debug, PartialEq)]
pub struct FileModuleKorok<'a> {
    pub ast: &'a syn::ItemMod,
    pub attributes: Attributes<'a>,
    pub items: Vec<ItemKorok<'a>>,
    pub node: Option<Node>,
    pub store: &'a FileModuleStore,
}

impl<'a> FileModuleKorok<'a> {
    pub fn parse(ast: &'a syn::ItemMod, store: &'a FileModuleStore) -> CodamaResult<Self> {
        if ast.content.is_some() {
            return Err(syn::Error::new_spanned(
                ast,
                "Module has content, it should be parsed with ModuleKorok",
            )
            .into());
        }

        Ok(Self {
            ast,
            attributes: Attributes::parse(&ast.attrs)?,
            items: ItemKorok::parse_all(&store.file.items, &store.file_modules, &mut 0)?,
            node: None,
            store,
        })
    }
}

impl Korok for FileModuleKorok<'_> {
    fn node(&self) -> &Option<Node> {
        &self.node
    }

    fn set_node(&mut self, node: Option<Node>) {
        self.node = node;
    }
}
