use codama_macros::codama;

#[codama(type = boolean)]
#[codama(type = boolean())]
#[codama(type = boolean(integer(u32)))]
#[codama(type = boolean(size = integer(u32)))]
pub struct Test;

fn main() {}
