use crate::{NestedTypeNodeTrait, TypeNodeEnumTrait, TypeNodeTrait};
use codama_nodes_derive::{Node, TypeNode};

#[derive(Debug, Node, TypeNode)]
pub struct PostOffsetTypeNode<T: TypeNodeEnumTrait> {
    // Data.
    pub offset: i32,
    pub strategy: PostOffsetStrategy,

    // Children.
    pub r#type: T,
}

impl<T: TypeNodeEnumTrait> PostOffsetTypeNode<T> {
    pub fn new<U>(r#type: U, strategy: PostOffsetStrategy, offset: i32) -> Self
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
        Self::new(r#type, PostOffsetStrategy::Absolute, offset)
    }

    pub fn padded<U>(r#type: U, offset: i32) -> Self
    where
        U: Into<T>,
    {
        Self::new(r#type, PostOffsetStrategy::Padded, offset)
    }

    pub fn pre_offset<U>(r#type: U, offset: i32) -> Self
    where
        U: Into<T>,
    {
        Self::new(r#type, PostOffsetStrategy::PreOffset, offset)
    }

    pub fn relative<U>(r#type: U, offset: i32) -> Self
    where
        U: Into<T>,
    {
        Self::new(r#type, PostOffsetStrategy::Relative, offset)
    }
}

impl<T: TypeNodeEnumTrait, U: TypeNodeTrait> NestedTypeNodeTrait<U> for PostOffsetTypeNode<T>
where
    T: NestedTypeNodeTrait<U>,
{
    fn get_nested_type_node(&self) -> &U {
        self.r#type.get_nested_type_node()
    }
}

#[derive(Debug)]
pub enum PostOffsetStrategy {
    Absolute,
    Padded,
    PreOffset,
    Relative,
}
