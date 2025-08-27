use crate::{ItemKorok, KorokTrait};
use codama_attributes::Attributes;
use codama_errors::{combine_errors, CodamaError, CodamaResult};
use codama_nodes::Node;
use codama_stores::FileModuleStore;
use codama_syn_helpers::extensions::*;

#[derive(Debug, PartialEq)]
pub struct FileModuleKorok<'a> {
    pub ast: &'a syn::ItemMod,
    pub attributes: Attributes<'a>,
    pub file_attributes: Attributes<'a>,
    pub items: Vec<ItemKorok<'a>>,
    pub node: Option<Node>,
    pub store: &'a FileModuleStore,
}

impl<'a> FileModuleKorok<'a> {
    pub fn parse(item: &'a syn::Item, store: &'a FileModuleStore) -> CodamaResult<Self> {
        let syn::Item::Mod(ast) = item else {
            return Err(item.error("Expected an module").into());
        };
        if ast.content.is_some() {
            return Err(syn::Error::new_spanned(
                ast,
                "Module has content, it should be parsed with ModuleKorok",
            )
            .into());
        }

        let (attributes, file_attributes, items) = combine_errors!(
            Attributes::parse(&ast.attrs, item.into()).map_err(CodamaError::from),
            Attributes::parse(&store.file.attrs, (&store.file).into()).map_err(CodamaError::from),
            ItemKorok::parse_all(&store.file.items, &store.file_modules, &mut 0),
        )?;
        Ok(Self {
            ast,
            attributes,
            file_attributes,
            items,
            node: None,
            store,
        })
    }
}

impl KorokTrait for FileModuleKorok<'_> {
    fn node(&self) -> &Option<Node> {
        &self.node
    }

    fn set_node(&mut self, node: Option<Node>) {
        self.node = node;
    }

    fn attributes(&self) -> Option<&Attributes<'_>> {
        Some(&self.attributes)
    }
}
