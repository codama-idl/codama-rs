use std::fmt::Debug;

pub trait TypeNodeTrait: Debug + PartialEq + Clone {}

pub trait TypeNodeEnumTrait: Debug + PartialEq + Clone {}

pub trait NestedTypeNodeTrait<T: TypeNodeTrait>: Debug + PartialEq + Clone {
    fn get_nested_type_node(&self) -> &T;
}
