use codama_macros::codama;

#[codama(type = fixed_point(integer(u64), scale = 9, display = amount(decimals = 9, unit = "SOL")))]
pub struct Test;

fn main() {}
