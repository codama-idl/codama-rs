use codama_macros::codama;

#[codama(type = field(number(u8), "age", number(u32)))]
pub struct Test;

#[codama(type = field(type = number(u8), name = "age", type = number(u32)))]
pub struct TestExplicit;

fn main() {}
