use std::fmt::Debug;

pub trait NodeTrait:
    Debug + PartialEq + Clone + serde::Serialize + for<'de> serde::Deserialize<'de>
{
    const KIND: &'static str;

    fn kind(&self) -> &'static str {
        Self::KIND
    }
}

pub trait NodeUnionTrait:
    Debug + PartialEq + Clone + serde::Serialize + for<'de> serde::Deserialize<'de>
{
    fn kind(&self) -> &'static str;
}
