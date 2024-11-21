use std::fmt::Debug;

pub trait NodeTrait:
    Debug + PartialEq + Clone + serde::Serialize + for<'de> serde::Deserialize<'de>
{
    const KIND: &'static str;
}
