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

    #[error("Invalid decimal: {0}")]
    InvalidDecimal(String),

    #[error("Invalid identifier: {0}")]
    InvalidIdentifier(String),

    #[error("Invalid namespace: {0}")]
    InvalidNamespace(String),

    #[error("Invalid path: {0}")]
    InvalidPath(String),

    #[error("Invalid number format: {0}")]
    InvalidNumberFormat(String),

    #[error("Invalid endian: {0}")]
    InvalidEndian(String),

    #[error("Invalid base: {0}")]
    InvalidBase(u64),

    #[error("Invalid attribute, Expected {expected}, got {actual}")]
    InvalidAttribute { expected: String, actual: String },

    #[error("Invalid Codama directive, Expected {expected}, got {actual}")]
    InvalidCodamaDirective { expected: String, actual: String },

    #[error("Unresolved directive: {namespace}::{name}")]
    UnresolvedDirective { namespace: String, name: String },

    #[error("Two different `{kind}` share the identifier `{identifier}` in {location}")]
    NodeMergeConflict {
        kind: String,
        identifier: String,
        location: String,
    },

    #[error("Two different plugins share the namespace `{namespace}` in {location}")]
    PluginMergeConflict { namespace: String, location: String },

    /// Every error that the check of the final root node finds, one message per line.
    #[error("{}", .0.join("\n"))]
    InvalidRootNode(Vec<String>),
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
