use codama::codama;

#[codama(node(number_type(u32)))]
#[codama(node(number_type(u32, be)))]
#[codama(node(number_type(u32, le)))]
#[codama(node(number_type(le, u32)))]
#[codama(node(number_type(format = u32, endian = le)))]
#[codama(node(number_type(endian = le, format = u32)))]
pub struct Test;

fn main() {}
