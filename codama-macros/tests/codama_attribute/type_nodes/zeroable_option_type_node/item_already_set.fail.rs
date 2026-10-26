use codama_macros::codama;

#[codama(type = zeroable_option(integer(u8), integer(u16)))]
pub struct Test;

#[codama(type = zeroable_option(item = integer(u8), item = integer(u16)))]
pub struct TestExplicit;

fn main() {}
