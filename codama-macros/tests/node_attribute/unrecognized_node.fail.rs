use codama_macros::node;

#[node(unrecognizedNode(foo = 42))]
pub struct Foo(usize);

fn main() {}
