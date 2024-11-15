use crate::{NestedTypeNodeTrait, TypeNodeEnumTrait, TypeNodeTrait};
use codama_nodes_derive::{Node, TypeNode};

#[derive(Node, TypeNode, Debug, PartialEq)]
pub struct PreOffsetTypeNode<T: TypeNodeEnumTrait> {
    // Data.
    pub offset: i32,
    pub strategy: PreOffsetStrategy,

    // Children.
    pub r#type: T,
}

impl<T: TypeNodeEnumTrait> PreOffsetTypeNode<T> {
    pub fn new<U>(r#type: U, strategy: PreOffsetStrategy, offset: i32) -> Self
    where
        U: Into<T>,
    {
        Self {
            r#type: r#type.into(),
            strategy,
            offset,
        }
    }

    pub fn absolute<U>(r#type: U, offset: i32) -> Self
    where
        U: Into<T>,
    {
        Self::new(r#type, PreOffsetStrategy::Absolute, offset)
    }

    pub fn padded<U>(r#type: U, offset: i32) -> Self
    where
        U: Into<T>,
    {
        Self::new(r#type, PreOffsetStrategy::Padded, offset)
    }

    pub fn relative<U>(r#type: U, offset: i32) -> Self
    where
        U: Into<T>,
    {
        Self::new(r#type, PreOffsetStrategy::Relative, offset)
    }
}

impl<T: TypeNodeEnumTrait, U: TypeNodeTrait> NestedTypeNodeTrait<U> for PreOffsetTypeNode<T>
where
    T: NestedTypeNodeTrait<U>,
{
    fn get_nested_type_node(&self) -> &U {
        self.r#type.get_nested_type_node()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PreOffsetStrategy {
    Absolute,
    Padded,
    Relative,
}
