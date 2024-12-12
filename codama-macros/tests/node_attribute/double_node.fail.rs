use codama::codama;

#[codama(node(number_type(u32, le), public_key_type()))]
pub struct Foo(usize);

fn main() {}
