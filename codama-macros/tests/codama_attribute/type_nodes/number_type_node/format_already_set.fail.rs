use codama_macros::codama;

#[codama(type = number(u32, u64))]
pub struct Test;

#[codama(type = number(format = u32, format = u64))]
pub struct TestExplicit;

fn main() {}
