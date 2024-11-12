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
    pub fn new(r#type: T, strategy: PostOffsetStrategy, offset: usize) -> Self {
        Self {
            r#type,
            strategy,
            offset,
        }
    }

    pub fn absolute(r#type: T, offset: usize) -> Self {
        Self::new(r#type, PostOffsetStrategy::Absolute, offset)
    }

    pub fn padded(r#type: T, offset: usize) -> Self {
        Self::new(r#type, PostOffsetStrategy::Padded, offset)
    }

    pub fn pre_offset(r#type: T, offset: usize) -> Self {
        Self::new(r#type, PostOffsetStrategy::PreOffset, offset)
    }

    pub fn relative(r#type: T, offset: usize) -> Self {
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
