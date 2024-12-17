use codama::codama;

#[codama(type = number(u32))]
#[codama(type = number(u32, be))]
#[codama(type = number(u32, le))]
#[codama(type = number(le, u32))]
#[codama(type = number(format = u32, endian = le))]
#[codama(type = number(endian = le, format = u32))]
pub struct Test;

fn main() {}
