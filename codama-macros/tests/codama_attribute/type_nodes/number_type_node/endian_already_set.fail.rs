use codama::codama;

#[codama(type = number(be, le, u32))]
pub struct Test;

#[codama(type = number(endian = be, endian = le, format = u32))]
pub struct TestExplicit;

fn main() {}
