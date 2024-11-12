use codama_nodes_derive::IntoEnum;

pub trait SomeTrait {}
impl SomeTrait for u32 {}

pub struct NumberTypeNode<T: SomeTrait> {
    pub value: T,
}
pub struct StringTypeNode<T: SomeTrait> {
    pub value: T,
}

#[derive(IntoEnum)]
pub enum TypeNode<T: SomeTrait> {
    Number(NumberTypeNode<T>),
    String(StringTypeNode<T>),
}

fn main() {
    let number: TypeNode<u32> = NumberTypeNode { value: 42 }.into();
    assert!(matches!(number, TypeNode::Number(_)));

    let string: TypeNode<u32> = StringTypeNode { value: 42 }.into();
    assert!(matches!(string, TypeNode::String(_)));
}
