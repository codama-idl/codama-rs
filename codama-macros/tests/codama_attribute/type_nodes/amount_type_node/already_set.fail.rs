use codama_macros::codama;

#[codama(type = amount(number(u64), decimals = 9, decimals = 6))]
pub struct Test;

fn main() {}
