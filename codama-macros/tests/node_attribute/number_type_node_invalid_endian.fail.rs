use codama_macros::node;

#[node(number_type(u32, endian = invalid))]
pub struct Foo(usize);

fn main() {}
