use codama_errors::CodamaError;
use codama_visitors_core::NodePath;
use std::fmt;

/// The severity of a [`ValidationItem`], from the least to the most severe, like `LOG_LEVELS` in the
/// JavaScript library. Levels compare by severity, so `LogLevel::Warn < LogLevel::Error`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LogLevel {
    Debug,
    Trace,
    Info,
    Warn,
    Error,
}

impl LogLevel {
    /// The name of the level, such as `warn`.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Debug => "debug",
            Self::Trace => "trace",
            Self::Info => "info",
            Self::Warn => "warn",
            Self::Error => "error",
        }
    }
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// A diagnostic reported when validating a Codama tree.
///
/// The `path` leads to the node the diagnostic is about, which is its last node. When the diagnostic
/// corresponds to a [`CodamaError`], that error is kept as its `cause`, so callers can match it rather
/// than its message.
#[derive(Debug)]
pub struct ValidationItem<'a> {
    pub level: LogLevel,
    pub message: String,
    pub path: NodePath<'a>,
    pub cause: Option<CodamaError>,
}

/// Creates a [`ValidationItem`] about the last node of `path`, without a cause.
///
/// # Panics
///
/// Panics when `path` is empty, as a diagnostic must be about a node.
///
/// ```
/// use codama_nodes::{NodeRef, TupleTypeNode};
/// use codama_validators::{validation_item, LogLevel};
/// use codama_visitors_core::NodePath;
///
/// let tuple = TupleTypeNode::new(vec![]);
/// let item = validation_item(LogLevel::Warn, "Tuple has no items.", NodePath::from(vec![NodeRef::from(&tuple)]));
/// assert_eq!(item.to_string(), "[warn] Tuple has no items. at [tupleTypeNode]");
/// ```
pub fn validation_item<'a>(
    level: LogLevel,
    message: impl Into<String>,
    path: NodePath<'a>,
) -> ValidationItem<'a> {
    assert!(
        !path.is_empty(),
        "A validation item must be about a node, but its path is empty"
    );
    ValidationItem {
        level,
        message: message.into(),
        path,
        cause: None,
    }
}

impl ValidationItem<'_> {
    /// Keeps `cause` as the error this diagnostic corresponds to.
    pub fn with_cause(self, cause: CodamaError) -> Self {
        Self {
            cause: Some(cause),
            ..self
        }
    }
}

impl fmt::Display for ValidationItem<'_> {
    /// Displays the level, the message and the path of the item, such as
    /// `[error] Program has no public key. at [programNode]token`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {} at {}", self.level, self.message, self.path)
    }
}

impl PartialEq for ValidationItem<'_> {
    /// Compares the level, message and path. A cause is compared by its message, as errors do not
    /// implement `PartialEq`.
    fn eq(&self, other: &Self) -> bool {
        self.level == other.level
            && self.message == other.message
            && self.path == other.path
            && self.cause.as_ref().map(ToString::to_string)
                == other.cause.as_ref().map(ToString::to_string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use codama_nodes::{NodeRef, TupleTypeNode};

    #[test]
    fn levels_compare_by_severity() {
        assert!(LogLevel::Debug < LogLevel::Trace);
        assert!(LogLevel::Trace < LogLevel::Info);
        assert!(LogLevel::Info < LogLevel::Warn);
        assert!(LogLevel::Warn < LogLevel::Error);
    }

    #[test]
    fn has_no_cause_by_default() {
        let tuple = TupleTypeNode::new(vec![]);
        let item = validation_item(
            LogLevel::Warn,
            "Tuple has no items.",
            NodePath::from(vec![NodeRef::from(&tuple)]),
        );
        assert!(item.cause.is_none());
        let item = item.with_cause(CodamaError::NodeNotFound);
        assert_eq!(item.cause.unwrap().to_string(), "Node not found");
    }

    #[test]
    #[should_panic(expected = "A validation item must be about a node")]
    fn refuses_an_empty_path() {
        validation_item(LogLevel::Error, "Oops.", NodePath::default());
    }
}
