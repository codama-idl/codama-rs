use codama_macros::codama;

#[codama(type = integer(u32, u64))]
pub struct Test;

#[codama(type = integer(format = u32, format = u64))]
pub struct TestExplicit;

fn main() {}
