use crate::CrateStore;
use codama_errors::CodamaResult;
use std::path::Path;

#[derive(Debug, PartialEq)]
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
