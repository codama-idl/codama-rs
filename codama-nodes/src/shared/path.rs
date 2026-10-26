use serde::{de, Deserialize, Serialize};
use std::{fmt, str::FromStr};

/// A path expression into nested data in the spec `path` grammar, such as `header.kind` or `[0].banana`.
/// The string keeps the value as written, it is never re-cased.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
#[serde(transparent)]
pub struct Path(String);

impl Path {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Path {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsePathError(String);

impl fmt::Display for ParsePathError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Invalid path: {}", self.0)
    }
}

impl std::error::Error for ParsePathError {}

impl TryFrom<String> for Path {
    type Error = ParsePathError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if is_path(&value) {
            Ok(Path(value))
        } else {
            Err(ParsePathError(value))
        }
    }
}

impl TryFrom<&str> for Path {
    type Error = ParsePathError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Path::try_from(value.to_string())
    }
}

impl FromStr for Path {
    type Err = ParsePathError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Path::try_from(value)
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

/// Strips a leading identifier `[A-Za-z_][A-Za-z0-9_]*` and returns the rest.
fn strip_identifier(value: &str) -> Option<&str> {
    let first_ok =
        value.starts_with(|character: char| character.is_ascii_alphabetic() || character == '_');
    let end = value
        .find(|character: char| !(character.is_ascii_alphanumeric() || character == '_'))
        .unwrap_or(value.len());
    first_ok.then(|| &value[end..])
}

/// Strips a leading index `"[" (0|[1-9][0-9]*) "]"` and returns the rest.
fn strip_index(value: &str) -> Option<&str> {
    let (integer, rest) = value.strip_prefix('[')?.split_once(']')?;
    let is_digits = !integer.is_empty() && integer.bytes().all(|byte| byte.is_ascii_digit());
    let is_integer = is_digits && (integer == "0" || !integer.starts_with('0'));
    is_integer.then_some(rest)
}

impl<'de> Deserialize<'de> for Path {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct PathVisitor;

        impl de::Visitor<'_> for PathVisitor {
            type Value = Path;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a path expression string")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Path::try_from(value)
                    .map_err(|_| E::invalid_value(de::Unexpected::Str(value), &self))
            }
        }

        deserializer.deserialize_str(PathVisitor)
    }
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
    fn try_from() {
        assert_eq!(Path::try_from("[0].kind").unwrap().as_str(), "[0].kind");
        assert_eq!(
            Path::try_from("[0].kind".to_string()).unwrap().as_str(),
            "[0].kind"
        );
        assert_eq!(
            Path::try_from("header..kind".to_string()),
            Err(ParsePathError("header..kind".to_string()))
        );
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
