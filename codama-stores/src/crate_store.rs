use cargo_toml::Manifest;
use codama_errors::CodamaResult;
use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::FileModuleStore;

#[derive(Debug, PartialEq)]
pub struct CrateStore {
    pub file: syn::File,
    pub manifest: Option<Manifest>,
    pub file_modules: Vec<FileModuleStore>,
    pub path: PathBuf,
}

impl CrateStore {
    pub fn load<P: AsRef<Path>>(path: P) -> CodamaResult<Self> {
        // Find and load the closest Cargo.toml file — a.k.a. the crate's manifest.
        let manifest_path = get_closest_manifest_path(path.as_ref())?;
        let mut manifest = Manifest::from_path(&manifest_path)?;
        manifest.complete_from_path(path.as_ref())?;

        // Find the crate's content from the manifest.
        let relative_product_path = get_product_path(&manifest)?;
        let product_path = manifest_path.parent().unwrap().join(relative_product_path);

        // Load the crate's content and parse it.
        let content = fs::read_to_string(&product_path)?;
        let file = syn::parse_file(&content)?;

        // Load all external modules from the crate's content.
        let modules = FileModuleStore::load_all(&product_path, &file.items)?;

        Ok(Self {
            file,
            manifest: Some(manifest),
            file_modules: modules,
            path: product_path.to_path_buf(),
        })
    }

    pub fn hydrate(tt: proc_macro2::TokenStream) -> CodamaResult<Self> {
        Ok(Self {
            file: syn::parse2::<syn::File>(tt)?,
            manifest: None,
            file_modules: Vec::new(),
            path: PathBuf::new(),
        })
    }
}

/// Given a path, get the closest available path to a Cargo.toml file.
/// E.g. "my/crate/Cargo.toml" returns "my/crate/Cargo.toml"
/// E.g. "my/crate" may return "my/crate/Cargo.toml"
/// E.g. "my/workspace/crate" may return "my/workspace/Cargo.toml"
pub fn get_closest_manifest_path<P: AsRef<Path>>(path: P) -> CodamaResult<PathBuf> {
    let mut current_path = path.as_ref().canonicalize()?;

    // If the initial path is a valid Cargo.toml file, return it.
    if current_path.ends_with("Cargo.toml") && current_path.is_file() {
        return Ok(current_path);
    }

    // Otherwise, search for the closest Cargo.toml file by moving up the directory tree.
    loop {
        let cargo_toml = current_path.join("Cargo.toml");
        if cargo_toml.is_file() {
            return Ok(cargo_toml);
        }

        // Move up one directory
        match current_path.parent() {
            Some(parent) => current_path = parent.to_path_buf(),
            None => break, // Reached the root directory.
        }
    }

    // If no Cargo.toml file was found, return an error.
    Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Cargo.toml not found").into())
}

fn get_product_path(manifest: &Manifest) -> CodamaResult<PathBuf> {
    let product = get_product_candidates(manifest)
        .iter()
        .filter_map(|product| product.path.as_ref())
        .next();

    match product {
        Some(path) => Ok(PathBuf::from(path)),
        None => Err(cargo_toml::Error::Other("No crate path found in Cargo.toml").into()),
    }
}

fn get_product_candidates(manifest: &Manifest) -> Vec<&cargo_toml::Product> {
    let mut candidates = Vec::new();
    if let Some(product) = &manifest.lib {
        candidates.push(product);
    }
    for product in &manifest.bin {
        candidates.push(product);
    }
    candidates
}
