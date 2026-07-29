use codama_macros::codama;

#[codama(type = amount(number(u64)))]
pub struct NumberOnlyTest;

#[codama(type = amount(number(u64), 9, "SOL"))]
pub struct ImplicitTest;

#[codama(type = amount(number = number(u64), decimals = 9, unit = "SOL"))]
pub struct ExplicitTest;

#[codama(type = amount(number(u64), 6))]
pub struct DecimalsOnlyTest;

fn main() {}
