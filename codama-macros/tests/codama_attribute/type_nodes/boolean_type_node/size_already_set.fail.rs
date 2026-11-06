use codama_macros::codama;

#[codama(type = boolean(integer(u32), integer(u32)))]
pub struct Test;

#[codama(type = boolean(size = integer(u32), size = integer(u32)))]
pub struct TestExplicit;

fn main() {}
