use super::string::{impl_constrained_string, strip_identifier, strip_index};
use crate::Identifier;
use codama_errors::CodamaError;
use serde::Serialize;

/// A path expression into nested data in the spec `path` grammar, such as `header.kind` or `[0].banana`.
/// The string keeps the value as written, it is never re-cased.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
#[serde(transparent)]
pub struct Path(String);

impl_constrained_string!(
    Path,
    is_path,
    CodamaError::InvalidPath,
    "a path expression string"
);

/// An identifier is a path with a single segment, which reads the field of that identifier.
impl From<Identifier> for Path {
    fn from(identifier: Identifier) -> Self {
        Self(identifier.as_str().to_string())
    }
}

/// A segment of a [`Path`]: a field identifier such as `config`, or an index such as `[0]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PathSegment<'p> {
    Field(&'p str),
    Index(usize),
}

impl std::fmt::Display for PathSegment<'_> {
    /// Displays the segment as written: `config` or `[0]`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Field(identifier) => f.write_str(identifier),
            Self::Index(index) => write!(f, "[{index}]"),
        }
    }
}

impl Path {
    /// The segments of the path, like `parsePath` in the JavaScript library.
    ///
    /// ```
    /// use codama_nodes::{Path, PathSegment};
    ///
    /// let path: Path = "config.fees[0]".parse().unwrap();
    /// assert_eq!(
    ///     path.segments(),
    ///     [PathSegment::Field("config"), PathSegment::Field("fees"), PathSegment::Index(0)]
    /// );
    /// ```
    pub fn segments(&self) -> Vec<PathSegment<'_>> {
        let mut segments = vec![];
        let mut rest = self.0.as_str();
        while !rest.is_empty() {
            rest = rest.strip_prefix('.').unwrap_or(rest);
            if let Some(after) = strip_index(rest) {
                let index = &rest[1..rest.len() - after.len() - 1];
                // The grammar bounds the digits, but an index beyond `usize` cannot select anything.
                segments.push(PathSegment::Index(index.parse().unwrap_or(usize::MAX)));
                rest = after;
            } else {
                let after = strip_identifier(rest).expect("a path is valid by construction");
                segments.push(PathSegment::Field(&rest[..rest.len() - after.len()]));
                rest = after;
            }
        }
        segments
    }

    /// The path has a `.` segment or an `[n]` index, such as `header.kind`, `fees[0]` or `[0]`.
    /// A plain identifier such as `amount` is not nested.
    pub fn is_nested(&self) -> bool {
        strip_identifier(&self.0) != Some("")
    }
}

/// Checks `value` against the spec `path` grammar.
/// For example `header.kind`, `config.fees[0]` or `[0].banana`.
/// See: @codama/spec/api/types.ts.
fn is_path(value: &str) -> bool {
    let mut rest = strip_identifier(value).or_else(|| strip_index(value));
    while let Some(remaining) = rest {
        if remaining.is_empty() {
            return true;
        }
        rest = match remaining.strip_prefix('.') {
            Some(segment) => strip_identifier(segment),
            None => strip_index(remaining),
        };
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID: [&str; 18] = [
        "discriminator",
        "accountState",
        "amount",
        "decimals",
        "authority",
        "header.kind",
        "mint.data",
        "mint.data.decimals",
        "config.fees[0]",
        "state.balances[0]",
        "[0].banana",
        "[0]",
        "a[0].b",
        "[0][1]",
        "a[10][2].b_c",
        "_private",
        "field_1.Inner2",
        "a",
    ];

    const INVALID: [&str; 22] = [
        "",
        ".a",
        "a.",
        "a..b",
        "a.[0]",
        "a.1",
        "[0]a",
        "[]",
        "[01]",
        "[-1]",
        "[+1]",
        "[a]",
        "a[",
        "a[0",
        "a]",
        "1a",
        "a b",
        " a",
        "a-b",
        "a\n",
        "\u{e9}t\u{e9}",
        "a.\u{e9}",
    ];

    #[test]
    fn from_str() {
        for value in VALID {
            let path = value.parse::<Path>().unwrap();
            assert_eq!(path.as_str(), value);
            assert_eq!(path.to_string(), value);
        }
    }

    #[test]
    fn from_str_rejects_invalid_paths() {
        for value in INVALID {
            let error = value.parse::<Path>().unwrap_err();
            assert_eq!(error.to_string(), format!("Invalid path: {value}"));
        }
    }

    #[test]
    fn from_static_str() {
        assert_eq!(Path::from("[0].kind").as_str(), "[0].kind");
    }

    #[test]
    #[should_panic(expected = "Invalid path: header..kind")]
    fn from_static_str_panics_on_an_invalid_path() {
        let _ = Path::from("header..kind");
    }

    #[test]
    fn try_from_string() {
        assert_eq!(
            Path::try_from("[0].kind".to_string()).unwrap().as_str(),
            "[0].kind"
        );
        assert!(matches!(
            Path::try_from("header..kind".to_string()),
            Err(CodamaError::InvalidPath(value)) if value == "header..kind"
        ));
    }

    #[test]
    fn segments() {
        let segments = |value: &str| {
            let path: Path = value.parse().unwrap();
            path.segments()
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
        };
        assert_eq!(
            segments("config.fees[0].amount"),
            ["config", "fees", "[0]", "amount"]
        );
        assert_eq!(segments("[12]"), ["[12]"]);
        assert_eq!(segments("[0][1]"), ["[0]", "[1]"]);
        assert_eq!(segments("a"), ["a"]);
        let path: Path = "a[10]".parse().unwrap();
        assert_eq!(
            path.segments(),
            [PathSegment::Field("a"), PathSegment::Index(10)]
        );
    }

    #[test]
    fn from_identifier() {
        let path = Path::from(Identifier::from("account_type"));
        assert_eq!(path.as_str(), "account_type");
        assert!(!path.is_nested());
    }

    #[test]
    fn is_nested() {
        for (value, expected) in [
            ("amount", false),
            ("_private", false),
            ("header.kind", true),
            ("fees[0]", true),
            ("[0]", true),
            ("[0].banana", true),
        ] {
            assert_eq!(Path::from(value).is_nested(), expected, "{value}");
        }
    }

    #[test]
    fn json_keeps_the_string() {
        for value in VALID {
            let json = serde_json::to_string(value).unwrap();
            let path = serde_json::from_str::<Path>(&json).unwrap();
            assert_eq!(path.as_str(), value);
            assert_eq!(serde_json::to_string(&path).unwrap(), json);
        }
    }

    #[test]
    fn from_json_rejects_invalid_paths() {
        for value in INVALID {
            let json = serde_json::to_string(value).unwrap();
            assert!(serde_json::from_str::<Path>(&json).is_err());
        }
        assert!(serde_json::from_str::<Path>("0").is_err());
    }

    #[test]
    fn from_json_error_keeps_the_position() {
        let error = serde_json::from_str::<Path>(r#""a..b""#).unwrap_err();
        assert_eq!(
            error.to_string(),
            "invalid value: string \"a..b\", expected a path expression string at line 1 column 6"
        );
    }
}
