use codama_nodes_derive::IntoEnum;

pub struct NumberTypeNode {}
pub struct StringTypeNode {}

#[derive(IntoEnum)]
pub enum TypeNode {
    Number(NumberTypeNode),
    String(StringTypeNode),
}

fn main() {
    let number: TypeNode = NumberTypeNode {}.into();
    assert!(matches!(number, TypeNode::Number(_)));

    let string: TypeNode = StringTypeNode {}.into();
    assert!(matches!(string, TypeNode::String(_)));
}
