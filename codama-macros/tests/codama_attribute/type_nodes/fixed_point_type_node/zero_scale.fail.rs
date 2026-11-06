use codama_macros::codama;

#[codama(type = fixed_point(integer(u64), scale = 0, unit = "USDC"))]
pub struct Test;

fn main() {}
