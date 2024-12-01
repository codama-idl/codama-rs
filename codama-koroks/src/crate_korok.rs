use crate::ItemKorok;
use cargo_toml::Manifest;
use codama_errors::CodamaResult;
use codama_nodes::Node;
use codama_stores::CrateStore;
use std::path::Path;

#[derive(Debug)]
pub struct CrateKorok<'a> {
    pub file: &'a syn::File,
    pub items: Vec<ItemKorok<'a>>,
    pub manifest: &'a Option<Manifest>,
    pub node: Option<Node>,
    pub path: &'a Path,
}

impl<'a> CrateKorok<'a> {
    pub fn parse(crate_store: &'a CrateStore) -> CodamaResult<Self> {
        Ok(Self {
            file: &crate_store.file,
            items: ItemKorok::parse_all(&crate_store.file.items, &crate_store.file_modules)?,
            manifest: &crate_store.manifest,
            node: None,
            path: &crate_store.path,
        })
    }
}
