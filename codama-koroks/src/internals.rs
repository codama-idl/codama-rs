#[derive(Debug)]
pub enum ParsingError {
    Filesystem(std::io::Error),
    Manifest(cargo_toml::Error),
    Compilation(syn::Error),
}

impl From<std::io::Error> for ParsingError {
    fn from(error: std::io::Error) -> Self {
        ParsingError::Filesystem(error)
    }
}

impl From<cargo_toml::Error> for ParsingError {
    fn from(error: cargo_toml::Error) -> Self {
        ParsingError::Manifest(error)
    }
}

impl From<syn::Error> for ParsingError {
    fn from(error: syn::Error) -> Self {
        ParsingError::Compilation(error)
    }
}

pub type ParsingResult<T> = Result<T, ParsingError>;
