use crate::Base;
use serde::{de, Deserialize, Serialize};

// Spec writes `base` as JSON number `2` or `10`, which serde enum derives cannot express.
impl Serialize for Base {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Base::Base2 => serializer.serialize_u8(2),
            Base::Base10 => serializer.serialize_u8(10),
        }
    }
}

impl<'de> Deserialize<'de> for Base {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct BaseVisitor;

        impl de::Visitor<'_> for BaseVisitor {
            type Value = Base;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("the number 2 or 10")
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match value {
                    2 => Ok(Base::Base2),
                    10 => Ok(Base::Base10),
                    _ => Err(E::invalid_value(de::Unexpected::Unsigned(value), &self)),
                }
            }
        }

        deserializer.deserialize_u64(BaseVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_json() {
        assert_eq!(serde_json::to_string(&Base::Base2).unwrap(), "2");
        assert_eq!(serde_json::to_string(&Base::Base10).unwrap(), "10");
    }

    #[test]
    fn from_json() {
        assert_eq!(serde_json::from_str::<Base>("2").unwrap(), Base::Base2);
        assert_eq!(serde_json::from_str::<Base>("10").unwrap(), Base::Base10);
    }

    #[test]
    fn from_json_rejects_other_values() {
        for json in ["3", "-2", "2.0", r#""2""#] {
            assert!(serde_json::from_str::<Base>(json).is_err(), "{json}");
        }
    }
}
