use proc_macro2::TokenStream;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CodamaError {
    #[error("{0}")]
    Filesystem(#[from] std::io::Error),

    #[error("Could not parse Cargo.toml files: {0}")]
    Manifest(#[from] cargo_toml::Error),

    #[error("{0}")]
    Compilation(#[from] syn::Error),

    #[error("Could not parse JSON files: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Could not convert node from `{from}` into `{into}`")]
    InvalidNodeConversion { from: String, into: String },

    #[error("Unexpected node: expected `{expected}`, found `{actual}`")]
    UnexpectedNode { expected: String, actual: String },

    #[error("Node not found")]
    NodeNotFound,

    #[error("Invalid encoding: {0}")]
    InvalidBytesEncoding(String),

    #[error("Invalid number format: {0}")]
    InvalidNumberFormat(String),

    #[error("Invalid endian: {0}")]
    InvalidEndian(String),

    #[error("Invalid attribute, Expected {expected}, got {actual}")]
    InvalidAttribute { expected: String, actual: String },

    #[error("Invalid Codama directive, Expected {expected}, got {actual}")]
    InvalidCodamaDirective { expected: String, actual: String },
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
