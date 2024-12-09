use codama_macros::node;

#[node(numberTypeNode(u32, le), publicKeyTypeNode())]
pub struct Foo(usize);

fn main() {}
