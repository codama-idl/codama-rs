use codama_macros::codama;

#[codama(type = option(number(u8), number(u16)))]
pub struct Test;

#[codama(type = option(item = number(u8), item = number(u16)))]
pub struct TestExplicit;

fn main() {}
