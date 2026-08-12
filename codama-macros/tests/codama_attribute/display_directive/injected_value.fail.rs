use codama_macros::codama;

#[codama(display(amount(decimals = injected("mint_decimals"))))]
pub struct Test;

fn main() {}
