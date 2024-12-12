use codama::codama;

#[codama(node(number_type(u32, unrecognized = 42, le)))]
pub struct Foo(usize);

fn main() {}
