use codama_macros::codama;

#[codama(type = boolean)]
#[codama(type = boolean())]
#[codama(type = boolean(number(u32)))]
#[codama(type = boolean(size = number(u32)))]
pub struct Test;

fn main() {}
