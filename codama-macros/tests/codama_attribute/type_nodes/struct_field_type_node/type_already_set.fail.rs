use codama_macros::codama;

#[codama(type = field(integer(u8), "age", integer(u32)))]
pub struct Test;

#[codama(type = field(type = integer(u8), name = "age", type = integer(u32)))]
pub struct TestExplicit;

fn main() {}
