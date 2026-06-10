use crate::IsSigner;
use serde::{Deserialize, Serialize};

impl From<bool> for IsSigner {
    fn from(value: bool) -> Self {
        match value {
            false => Self::False,
            true => Self::True,
        }
    }
}

// The spec's literalUnion doesn't carry a notion of "default
// variant", so the generated `IsSigner` shell has no
// `#[derive(Default)]`. The default member is a Rust-side choice and
// lives here.
#[allow(clippy::derivable_impls)]
impl Default for IsSigner {
    fn default() -> Self {
        IsSigner::False
    }
}

// `IsSigner` serialises to a heterogeneous JSON value: real
// booleans `true`/`false` for the boolean variants and the string
// `"either"` for the third. Serde's derive + `rename_all` machinery
// can't express that — `rename` only produces strings — so the
// Serialize/Deserialize impls are hand-written. The generator emits
// the enum shell with non-serde derives only so these impls don't
// clash with derived ones.
impl Serialize for IsSigner {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            IsSigner::False => serializer.serialize_bool(false),
            IsSigner::True => serializer.serialize_bool(true),
            IsSigner::Either => serializer.serialize_str("either"),
        }
    }
}

impl<'de> Deserialize<'de> for IsSigner {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct IsSignerVisitor;

        impl serde::de::Visitor<'_> for IsSignerVisitor {
            type Value = IsSigner;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a boolean or the string 'either'")
            }

            fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(IsSigner::from(value))
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "either" => Ok(IsSigner::Either),
                    _ => Err(E::custom(format!("unexpected value: {value}"))),
                }
            }
        }

        deserializer.deserialize_any(IsSignerVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_json() {
        assert_eq!(serde_json::to_string(&IsSigner::True).unwrap(), "true");
        assert_eq!(serde_json::to_string(&IsSigner::False).unwrap(), "false");
        assert_eq!(
            serde_json::to_string(&IsSigner::Either).unwrap(),
            "\"either\""
        );
    }

    #[test]
    fn from_json() {
        assert_eq!(
            serde_json::from_str::<IsSigner>("true").unwrap(),
            IsSigner::True
        );
        assert_eq!(
            serde_json::from_str::<IsSigner>("false").unwrap(),
            IsSigner::False
        );
        assert_eq!(
            serde_json::from_str::<IsSigner>("\"either\"").unwrap(),
            IsSigner::Either
        );
    }

    #[test]
    fn from_bool() {
        assert_eq!(IsSigner::from(true), IsSigner::True);
        assert_eq!(IsSigner::from(false), IsSigner::False);
    }

    #[test]
    fn default_is_false() {
        assert_eq!(IsSigner::default(), IsSigner::False);
    }
}
