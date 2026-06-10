use serde::{Deserialize, Serialize};

/// How an attribute that carries a default value is exposed in generated APIs.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DefaultValueStrategy {
    /// The attribute is not exposed as a parameter in the generated API; the default value is always used.
    Omitted,
    /// The attribute is exposed as an optional parameter; callers may override the default value.
    Optional,
}
