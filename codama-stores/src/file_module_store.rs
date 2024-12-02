use codama_errors::CodamaResult;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct FileModuleStore {
    pub file: syn::File,
    pub item_index: usize,
    pub file_modules: Vec<FileModuleStore>,
    pub path: PathBuf,
}

impl FileModuleStore {
    pub fn load_all_from(path: &Path, items: &Vec<syn::Item>) -> CodamaResult<Vec<Self>> {
        find_nested_file_modules(items)
            .iter()
            .enumerate()
            .map(|(item_index, &item)| FileModuleStore::load_from(&path, item, item_index))
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
            file_modules: modules,
            path,
        })
    }
}

fn find_nested_file_modules(items: &Vec<syn::Item>) -> Vec<&syn::ItemMod> {
    items
        .iter()
        .filter_map(|item| match item {
            syn::Item::Mod(syn::ItemMod {
                content: Some((_, items)),
                ..
            }) => Some(find_nested_file_modules(items)),
            syn::Item::Mod(item_mod) => Some(vec![item_mod]),
            _ => None,
        })
        .flatten()
        .collect()
}
