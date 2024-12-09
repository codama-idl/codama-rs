use codama_macros::node;

#[node(unrecognized_node(foo = 42))]
pub struct Foo(usize);

fn main() {}
