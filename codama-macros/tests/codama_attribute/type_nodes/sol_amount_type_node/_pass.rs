use codama_macros::codama;

#[codama(type = sol_amount(number(u64)))]
pub struct ImplicitTest;

#[codama(type = sol_amount(number = number(u64)))]
pub struct ExplicitTest;

fn main() {}
