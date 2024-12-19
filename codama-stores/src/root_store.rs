use crate::CrateStore;
use codama_errors::{CodamaResult, IteratorCombineErrors};
use std::path::Path;

#[derive(Debug, PartialEq)]
pub struct RootStore {
    pub crates: Vec<CrateStore>,
}

impl RootStore {
    pub fn load<P: AsRef<Path>>(path: P) -> CodamaResult<Self> {
        Self::load_all(&[path])
    }

    pub fn load_all<P: AsRef<Path>>(paths: &[P]) -> CodamaResult<Self> {
        Ok(Self {
            crates: paths
                .iter()
                .map(|path| CrateStore::load(path))
                .collect_and_combine_errors()?,
        })
    }

    pub fn hydrate(tt: proc_macro2::TokenStream) -> CodamaResult<Self> {
        Ok(Self {
            crates: vec![CrateStore::hydrate(tt)?],
        })
    }
}
