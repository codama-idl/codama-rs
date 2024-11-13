use std::fmt::Debug;

pub trait NodeTrait: Debug {
    const KIND: &'static str;
}
