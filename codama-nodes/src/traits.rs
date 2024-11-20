use std::fmt::Debug;

pub trait NodeTrait: Debug + PartialEq + Clone {
    const KIND: &'static str;
}
