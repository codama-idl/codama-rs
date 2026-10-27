use codama_macros::codama;

#[codama(type = fixed_point(integer(u64, unit = "USDC"), scale = 6))]
pub struct Test;

fn main() {}
