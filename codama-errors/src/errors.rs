use proc_macro2::TokenStream;
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
    #[error("Unexpected node: expected `{expected}`, found `{actual}`")]
    UnexpectedNode { expected: String, actual: String },
    #[error("Node not found")]
    NodeNotFound,
}

pub type CodamaResult<T> = Result<T, CodamaError>;

impl CodamaError {
    pub fn to_compile_error(&self) -> TokenStream {
        match self {
            CodamaError::Compilation(error) => error.to_compile_error(),
            _ => TokenStream::new(),
        }
    }

    pub fn into_compile_error(self) -> TokenStream {
        self.to_compile_error()
    }
}
