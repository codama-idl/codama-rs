use codama::codama;

#[codama(node(number_type(u32, endian = invalid)))]
pub struct Foo(usize);

fn main() {}
