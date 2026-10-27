use codama_macros::codama;

#[codama(type = fixed_point(integer(u64), scale = 6, foo = 42))]
pub struct Test;

#[codama(type = fixed_point(integer(u64), 6, "USDC"))]
pub struct PositionalTest;

fn main() {}
