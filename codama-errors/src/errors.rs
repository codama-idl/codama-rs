use proc_macro2::TokenStream;
use thiserror::Error;

/// Variants are added in minor versions, so matches outside this crate need a wildcard arm.
#[derive(Error, Debug)]
#[non_exhaustive]
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

    #[error("{}", invalid_identifier_message(.0))]
    InvalidIdentifier(String),

    #[error("Cannot derive an identifier from `{name}`: {reason}. Set one explicitly with `identifier = \"...\"`")]
    InvalidRustName { name: String, reason: String },

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

    #[error("Could not find the node linked from {path}.")]
    LinkedNodeNotFound { path: String },

    #[error("Expected a node but the `{kind}` was deleted")]
    DeletedNode { kind: String },

    #[error("Injected value [{key}] is not provided and has no fallback at {path}. Values injected into a defined type are provided by the instruction that uses it.")]
    InjectedValueNotProvided { key: String, path: String },

    #[error("The value provided at {path} is not one of [{expected_kinds}].")]
    InvalidProvidedValue {
        expected_kinds: String,
        path: String,
    },

    #[error("Expected node of kind [{expected_kinds}], got {path}.")]
    UnexpectedNodeKind {
        expected_kinds: String,
        path: String,
    },

    #[error("Invalid placeholder [{placeholder}] in interpolated intent [{template}]. Placeholders are `${{data.<path>}}`, `${{accounts.<account>}}` or `${{accounts.<account>.data[.<path>]}}`.")]
    InvalidInterpolatedIntentPlaceholder {
        placeholder: String,
        template: String,
    },

    #[error("Circular dependency detected when resolving the default values of instruction {path}. Got the following dependency cycle [{formatted_cycle}].")]
    CyclicDependencyDetectedWhenResolvingInstructionDefaultValues {
        path: String,
        formatted_cycle: String,
        /// The path of each input of the cycle, the first one repeated at the end.
        cycle: Vec<String>,
    },

    #[error("Dependency [{dependency_name}] of kind [{dependency_kind}] is not a valid dependency of [{dependent}] in instruction {path}.")]
    InvalidInstructionDefaultValueDependency {
        dependency_kind: String,
        dependency_name: String,
        dependent: String,
        path: String,
    },

    #[error("Cannot use optional account [{seed_value_name}] as PDA seed {path}.")]
    CannotUseOptionalAccountAsPdaSeedValue {
        seed_value_name: String,
        path: String,
    },

    #[error("Cannot resolve segment [{segment}] of path [{path_string}] from {path}.")]
    CannotResolvePath {
        segment: String,
        path_string: String,
        path: String,
    },

    #[error("Expected a positive integer number of ticks per second for {path}, got [{ticks_per_second}].")]
    InvalidTicksPerSecond { path: String, ticks_per_second: u32 },

    #[error("Expected a node path containing a program node, e.g. to resolve the links of its last node, got {path}.")]
    NodePathProgramMissing { path: String },

    #[error("Defined type {path} has no finite value, so no codec can encode or decode it.")]
    DefinedTypeHasNoFiniteValue { path: String },

    #[error("{}", failed_to_validate_node_message(.path, .histogram, .items))]
    FailedToValidateNode {
        path: String,
        histogram: String,
        items: Vec<String>,
    },
}

pub type CodamaResult<T> = Result<T, CodamaError>;

/// Lets an infallible conversion, such as a node into itself, fit where a `CodamaError` is expected.
impl From<std::convert::Infallible> for CodamaError {
    fn from(error: std::convert::Infallible) -> Self {
        match error {}
    }
}

/// The message of `CodamaError::FailedToValidateNode`, which lists its validation items below the summary,
/// one per line, as they carry the diagnostics that a caller needs to fix the node.
fn failed_to_validate_node_message(path: &str, histogram: &str, items: &[String]) -> String {
    let mut message = format!("Failed to validate node {path} [{histogram}].");
    for item in items {
        message.push_str("\n- ");
        message.push_str(item);
    }
    message
}

/// The message of `CodamaError::InvalidIdentifier`, which names an empty identifier rather than quoting it.
fn invalid_identifier_message(value: &str) -> String {
    const RULE: &str = "use letters, digits and underscores, not starting with a digit";
    match value {
        "" => format!("An identifier cannot be empty: {RULE}"),
        _ => format!("Invalid identifier `{value}`: {RULE}"),
    }
}

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
