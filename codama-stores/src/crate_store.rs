use cargo_toml::Manifest;
use codama_errors::CodamaResult;
use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::FileModuleStore;

#[derive(Debug)]
pub struct CrateStore {
    pub file: syn::File,
    pub manifest: Option<Manifest>,
    pub file_modules: Vec<FileModuleStore>,
    pub path: PathBuf,
}

impl CrateStore {
    pub fn load_from(path: &Path) -> CodamaResult<Self> {
        let content = fs::read_to_string(path)?;
        let file = syn::parse_file(&content)?;
        let manifest = Manifest::from_path(path)?;
        let modules = FileModuleStore::load_all_from(path, &file.items)?;

        Ok(Self {
            file,
            manifest: Some(manifest),
            file_modules: modules,
            path: path.to_path_buf(),
        })
    }

    pub fn populate_from(tt: proc_macro2::TokenStream) -> CodamaResult<Self> {
        Ok(Self {
            file: syn::parse2::<syn::File>(tt)?,
            manifest: None,
            file_modules: Vec::new(),
            path: PathBuf::new(),
        })
    }
}
