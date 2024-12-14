use crate::{ItemKorok, KorokTrait};
use codama_attributes::Attributes;
use codama_errors::{combine_errors, CodamaError, CodamaResult};
use codama_nodes::Node;
use codama_stores::FileModuleStore;

#[derive(Debug, PartialEq)]
pub struct ModuleKorok<'a> {
    pub ast: &'a syn::ItemMod,
    pub attributes: Attributes<'a>,
    pub items: Vec<ItemKorok<'a>>,
    pub node: Option<Node>,
}

impl<'a> ModuleKorok<'a> {
    pub fn parse(
        ast: &'a syn::ItemMod,
        file_modules: &'a [FileModuleStore],
        file_module_index: &mut usize,
    ) -> CodamaResult<Self> {
        let Some(content) = &ast.content else {
            return Err(syn::Error::new_spanned(
                ast,
                "Module has no content, it should be parsed with FileModuleKorok",
            )
            .into());
        };

        let (attributes, items) = combine_errors!(
            Attributes::parse(&ast.attrs).map_err(CodamaError::from),
            ItemKorok::parse_all(&content.1, file_modules, file_module_index),
        )?;
        Ok(Self {
            ast,
            attributes,
            items,
            node: None,
        })
    }
}

impl KorokTrait for ModuleKorok<'_> {
    fn node(&self) -> &Option<Node> {
        &self.node
    }

    fn set_node(&mut self, node: Option<Node>) {
        self.node = node;
    }

    fn attributes(&self) -> Option<&Attributes> {
        Some(&self.attributes)
    }
}
