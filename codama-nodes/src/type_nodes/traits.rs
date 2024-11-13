use std::fmt::Debug;

pub trait TypeNodeTrait: Debug + PartialEq {}

pub trait TypeNodeEnumTrait: Debug + PartialEq {}

pub trait NestedTypeNodeTrait<T: TypeNodeTrait>: Debug + PartialEq {
    fn get_nested_type_node(&self) -> &T;
}
