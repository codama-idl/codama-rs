use codama_macros::codama;

#[codama(type = integer(u64, display = amount(decimals = 9), display = amount(decimals = 6)))]
pub struct Test;

fn main() {}
