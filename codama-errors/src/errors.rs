#[derive(Debug)]
pub enum CodamaError {
    Filesystem(std::io::Error),
    Manifest(cargo_toml::Error),
    Compilation(syn::Error),
    // TODO: InvalidNodeConversion(..),
}

impl From<std::io::Error> for CodamaError {
    fn from(error: std::io::Error) -> Self {
        Self::Filesystem(error)
    }
}

impl From<cargo_toml::Error> for CodamaError {
    fn from(error: cargo_toml::Error) -> Self {
        Self::Manifest(error)
    }
}

impl From<syn::Error> for CodamaError {
    fn from(error: syn::Error) -> Self {
        Self::Compilation(error)
    }
}

pub type CodamaResult<T> = Result<T, CodamaError>;
