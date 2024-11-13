use std::fmt::Debug;

pub trait NodeTrait: Debug + PartialEq {
    const KIND: &'static str;
}
