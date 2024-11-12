use crate::Node;

use super::{NumberTypeNode, NumberTypeNodeFlag, TypeNodeEnumFlag};

#[derive(Debug)]
pub struct PostOffsetTypeNode<T: TypeNodeEnumFlag> {
    // Data.
    pub offset: usize,
    pub strategy: PostOffsetStrategy,

    // Children.
    pub r#type: T,
}

impl<T: TypeNodeEnumFlag> PostOffsetTypeNode<T> {
    pub fn new<U>(r#type: U, strategy: PostOffsetStrategy, offset: usize) -> Self
    where
        U: Into<T>,
    {
        Self {
            r#type: r#type.into(),
            strategy,
            offset,
        }
    }

    pub fn absolute<U>(r#type: U, offset: usize) -> Self
    where
        U: Into<T>,
    {
        Self::new(r#type, PostOffsetStrategy::Absolute, offset)
    }

    pub fn padded<U>(r#type: U, offset: usize) -> Self
    where
        U: Into<T>,
    {
        Self::new(r#type, PostOffsetStrategy::Padded, offset)
    }

    pub fn pre_offset<U>(r#type: U, offset: usize) -> Self
    where
        U: Into<T>,
    {
        Self::new(r#type, PostOffsetStrategy::PreOffset, offset)
    }

    pub fn relative<U>(r#type: U, offset: usize) -> Self
    where
        U: Into<T>,
    {
        Self::new(r#type, PostOffsetStrategy::Relative, offset)
    }
}

impl<T: TypeNodeEnumFlag> Node for PostOffsetTypeNode<T> {
    const KIND: &'static str = "postOffsetTypeNode";
}

impl<T: TypeNodeEnumFlag> NumberTypeNodeFlag for PostOffsetTypeNode<T>
where
    T: NumberTypeNodeFlag,
{
    fn get_number_type_node(&self) -> &NumberTypeNode {
        self.r#type.get_number_type_node()
    }
}

#[derive(Debug)]
pub enum PostOffsetStrategy {
    Absolute,
    Padded,
    PreOffset,
    Relative,
}
