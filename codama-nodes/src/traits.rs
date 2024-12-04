use codama_errors::CodamaResult;
use std::fmt::Debug;

pub trait NodeTrait:
    Debug + PartialEq + Clone + serde::Serialize + for<'de> serde::Deserialize<'de>
{
    const KIND: &'static str;

    fn kind(&self) -> &'static str {
        Self::KIND
    }

    fn to_json(&self) -> CodamaResult<String> {
        serde_json::to_string(&self).map_err(Into::into)
    }

    fn to_json_pretty(&self) -> CodamaResult<String> {
        serde_json::to_string_pretty(&self).map_err(Into::into)
    }

    fn from_json(json: &str) -> CodamaResult<Self> {
        serde_json::from_str(json).map_err(Into::into)
    }
}

pub trait NodeUnionTrait:
    Debug + PartialEq + Clone + serde::Serialize + for<'de> serde::Deserialize<'de>
{
    fn kind(&self) -> &'static str;
}
