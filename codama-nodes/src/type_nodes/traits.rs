use std::fmt::Debug;

pub trait TypeNodeTrait: Debug {}

pub trait TypeNodeEnumTrait: Debug {}

pub trait NestedTypeNodeTrait<T: TypeNodeTrait>: Debug {
    fn get_nested_type_node(&self) -> &T;
}
