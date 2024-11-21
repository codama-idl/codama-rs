use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum IsAccountSigner {
    True,
    False,
    Either,
}

impl From<bool> for IsAccountSigner {
    fn from(value: bool) -> Self {
        match value {
            true => Self::True,
            false => Self::False,
        }
    }
}

impl Serialize for IsAccountSigner {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            IsAccountSigner::True => serializer.serialize_bool(true),
            IsAccountSigner::False => serializer.serialize_bool(false),
            IsAccountSigner::Either => serializer.serialize_str("either"),
        }
    }
}

impl<'de> Deserialize<'de> for IsAccountSigner {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct IsAccountSignerVisitor;

        impl<'de> serde::de::Visitor<'de> for IsAccountSignerVisitor {
            type Value = IsAccountSigner;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a boolean or the string 'either'")
            }

            fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(IsAccountSigner::from(value))
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "either" => Ok(IsAccountSigner::Either),
                    _ => Err(E::custom(format!("unexpected value: {}", value))),
                }
            }
        }

        deserializer.deserialize_any(IsAccountSignerVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_json() {
        assert_eq!(
            serde_json::to_string(&IsAccountSigner::True).unwrap(),
            "true"
        );
        assert_eq!(
            serde_json::to_string(&IsAccountSigner::False).unwrap(),
            "false"
        );
        assert_eq!(
            serde_json::to_string(&IsAccountSigner::Either).unwrap(),
            "\"either\""
        );
    }

    #[test]
    fn from_json() {
        assert_eq!(
            serde_json::from_str::<IsAccountSigner>("true").unwrap(),
            IsAccountSigner::True
        );
        assert_eq!(
            serde_json::from_str::<IsAccountSigner>("false").unwrap(),
            IsAccountSigner::False
        );
        assert_eq!(
            serde_json::from_str::<IsAccountSigner>("\"either\"").unwrap(),
            IsAccountSigner::Either
        );
    }
}
