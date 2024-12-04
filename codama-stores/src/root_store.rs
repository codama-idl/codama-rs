use crate::CrateStore;
use codama_errors::CodamaResult;
use std::path::Path;

#[derive(Debug, PartialEq)]
pub struct RootStore {
    pub crates: Vec<CrateStore>,
}

impl RootStore {
    pub fn load(path: &Path) -> CodamaResult<Self> {
        Self::load_all(&[path])
    }

    pub fn load_all(paths: &[&Path]) -> CodamaResult<Self> {
        Ok(Self {
            crates: paths
                .iter()
                .map(|path| CrateStore::load(path))
                .collect::<CodamaResult<_>>()?,
        })
    }

    pub fn hydrate(tt: proc_macro2::TokenStream) -> CodamaResult<Self> {
        Ok(Self {
            crates: vec![CrateStore::hydrate(tt)?],
        })
    }
}
