use std::{
    fs,
    path::{Path, PathBuf},
};

use cargo_toml::Manifest;

use crate::internals::ParsingResult;

pub struct UnparsedRoot {
    pub crates: Vec<UnparsedCrate>,
}

impl UnparsedRoot {
    pub fn read(paths: &Vec<&Path>) -> ParsingResult<Self> {
        Ok(Self {
            crates: paths
                .iter()
                .map(|path| UnparsedCrate::read(path))
                .collect::<ParsingResult<_>>()?,
        })
    }
}

pub struct UnparsedCrate {
    pub file: syn::File,
    pub manifest: Manifest,
    pub modules: Vec<UnparsedModule>,
    pub path: PathBuf,
}

impl UnparsedCrate {
    pub fn read(path: &Path) -> ParsingResult<Self> {
        let content = fs::read_to_string(path)?;
        let file = syn::parse_file(&content)?;
        let manifest = Manifest::from_path(path)?;
        let modules = UnparsedModule::read_all(path, &file.items)?;

        Ok(Self {
            file,
            manifest,
            modules,
            path: path.to_path_buf(),
        })
    }
}

pub struct UnparsedModule {
    pub file: syn::File,
    pub item_index: usize,
    pub modules: Vec<UnparsedModule>,
    pub path: PathBuf,
}

impl UnparsedModule {
    pub fn read_all(path: &Path, items: &Vec<syn::Item>) -> ParsingResult<Vec<Self>> {
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
            .map(|(item_index, &item)| UnparsedModule::read(&path, item, item_index))
            .collect::<ParsingResult<Vec<_>>>()
    }

    pub fn read(path: &Path, item: &syn::ItemMod, item_index: usize) -> ParsingResult<Self> {
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
        let modules = Self::read_all(&path, &file.items)?;

        Ok(Self {
            file,
            item_index,
            modules,
            path,
        })
    }
}
