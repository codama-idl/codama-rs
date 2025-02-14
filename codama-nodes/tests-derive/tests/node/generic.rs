use codama_nodes_derive::Node;

pub trait HasKind {
    fn kind(&self) -> &'static str;
}
pub trait NodeTrait {
    const KIND: &'static str;
}

pub trait SomeTrait {}
impl SomeTrait for u32 {}

#[derive(Node)]
pub struct NumberTypeNode<T: SomeTrait> {
    pub value: T,
}

fn main() {
    assert_eq!(NumberTypeNode::<u32>::KIND, "numberTypeNode");
}
