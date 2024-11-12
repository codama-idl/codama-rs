use crate::{NestedTypeNodeTrait, NodeTrait, TypeNodeEnumTrait, TypeNodeTrait};

#[derive(Debug)]
pub struct PostOffsetTypeNode<T: TypeNodeEnumTrait> {
    // Data.
    pub offset: usize,
    pub strategy: PostOffsetStrategy,

    // Children.
    pub r#type: T,
}

impl<T: TypeNodeEnumTrait> PostOffsetTypeNode<T> {
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

impl<T: TypeNodeEnumTrait> TypeNodeTrait for PostOffsetTypeNode<T> {}
impl<T: TypeNodeEnumTrait> NodeTrait for PostOffsetTypeNode<T> {
    const KIND: &'static str = "postOffsetTypeNode";
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
