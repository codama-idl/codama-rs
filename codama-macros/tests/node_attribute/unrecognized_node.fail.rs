use codama::codama;

#[codama(node(unrecognized_node(foo = 42)))]
pub struct Foo(usize);

fn main() {}
