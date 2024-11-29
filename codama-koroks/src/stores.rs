use cargo_toml::Manifest;
use codama_errors::CodamaResult;
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct RootStore {
    pub crates: Vec<CrateStore>,
}

impl RootStore {
    pub fn load_from(paths: &Vec<&Path>) -> CodamaResult<Self> {
        Ok(Self {
            crates: paths
                .iter()
                .map(|path| CrateStore::load_from(path))
                .collect::<CodamaResult<_>>()?,
        })
    }

    pub fn populate_from(tt: proc_macro2::TokenStream) -> CodamaResult<Self> {
        Ok(Self {
            crates: vec![CrateStore::populate_from(tt)?],
        })
    }
}

#[derive(Debug)]
pub struct CrateStore {
    pub file: syn::File,
    pub manifest: Option<Manifest>,
    pub modules: Vec<ModuleStore>,
    pub path: PathBuf,
}

impl CrateStore {
    pub fn load_from(path: &Path) -> CodamaResult<Self> {
        let content = fs::read_to_string(path)?;
        let file = syn::parse_file(&content)?;
        let manifest = Manifest::from_path(path)?;
        let modules = ModuleStore::load_all_from(path, &file.items)?;

        Ok(Self {
            file,
            manifest: Some(manifest),
            modules,
            path: path.to_path_buf(),
        })
    }

    pub fn populate_from(tt: proc_macro2::TokenStream) -> CodamaResult<Self> {
        Ok(Self {
            file: syn::parse2::<syn::File>(tt)?,
            manifest: None,
            modules: Vec::new(),
            path: PathBuf::new(),
        })
    }
}

#[derive(Debug)]
pub struct ModuleStore {
    pub file: syn::File,
    pub item_index: usize,
    pub modules: Vec<ModuleStore>,
    pub path: PathBuf,
}

impl ModuleStore {
    pub fn load_all_from(path: &Path, items: &Vec<syn::Item>) -> CodamaResult<Vec<Self>> {
        let items = &items
            .iter()
            .filter_map(|item| match item {
                syn::Item::Mod(item_mod) if item_mod.content.is_none() => Some(item_mod),
                _ => None,
            })
            .collect::<Vec<_>>();

        items
            .iter()
            .enumerate()
            .map(|(item_index, &item)| ModuleStore::load_from(&path, item, item_index))
            .collect::<CodamaResult<Vec<_>>>()
    }

    pub fn load_from(path: &Path, item: &syn::ItemMod, item_index: usize) -> CodamaResult<Self> {
        let parent_directory = path.parent().unwrap();
        let filename = path.file_stem().unwrap().to_str().unwrap();
        let current_directory = parent_directory.join(filename);

        let candidates = vec![
            // If we are in a mod.rs or lib.rs file, the modules will be in a sibling directory.
            parent_directory.join(format!("{}.rs", item.ident)),
            parent_directory.join(format!("{}/mod.rs", item.ident)),
            // Otherwise, the modules will be in a child directory.
            current_directory.join(format!("{}.rs", item.ident)),
            current_directory.join(format!("{}/mod.rs", item.ident)),
        ];

        let path = candidates
            .into_iter()
            .find(|p| p.exists())
            .ok_or_else(|| syn::Error::new_spanned(&item, "could not read file"))?;
        let content = std::fs::read_to_string(&path)?;
        let file = syn::parse_file(&content)?;
        let modules = Self::load_all_from(&path, &file.items)?;

        Ok(Self {
            file,
            item_index,
            modules,
            path,
        })
    }
}
