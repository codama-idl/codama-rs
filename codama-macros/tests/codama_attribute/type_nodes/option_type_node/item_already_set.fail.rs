use codama_macros::codama;

#[codama(type = option(integer(u8), integer(u16)))]
pub struct Test;

#[codama(type = option(item = integer(u8), item = integer(u16)))]
pub struct TestExplicit;

fn main() {}
