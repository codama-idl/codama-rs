pub trait TypeNodeTrait {}

pub trait TypeNodeEnumTrait {}

pub trait NestedTypeNodeTrait<T: TypeNodeTrait> {
    fn get_nested_type_node(&self) -> &T;
}
