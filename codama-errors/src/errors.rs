use thiserror::Error;

#[derive(Error, Debug)]
pub enum CodamaError {
    #[error("IO error: {0}")]
    Filesystem(#[from] std::io::Error),
    #[error("Could not parse Cargo.toml files: {0}")]
    Manifest(#[from] cargo_toml::Error),
    #[error("A compilation error was identified via Syn: {0}")]
    Compilation(#[from] syn::Error),
    #[error("Could not convert node from `{from}` into `{into}`")]
    InvalidNodeConversion { from: String, into: String },
}

pub type CodamaResult<T> = Result<T, CodamaError>;
