use codama_macros::codama;

#[codama(type = integer(be, le, u32))]
pub struct Test;

#[codama(type = integer(endian = be, endian = le, format = u32))]
pub struct TestExplicit;

fn main() {}
